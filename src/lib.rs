use core::cmp::Ordering;

use proc_macro2::{Ident, TokenStream};
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Data, DeriveInput, Error, Fields, FieldsNamed, FieldsUnnamed, Token,
    TraitBound, Type,
};

struct DeriveWhere {
    bounds: Vec<TraitBound>,
    traits: Vec<Traits>,
}

impl Parse for DeriveWhere {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let bounds = Punctuated::<TraitBound, Token![,]>::parse_separated_nonempty(input)?
            .into_iter()
            .collect();
        <Token![;]>::parse(input)?;
        let traits = Punctuated::<Traits, Token![,]>::parse_terminated(input)?
            .into_iter()
            .collect();
        Ok(Self { bounds, traits })
    }
}

#[derive(Copy, Clone, Debug)]
enum Traits {
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
}

impl Parse for Traits {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        use Traits::*;
        let ident = Ident::parse(input)?;

        Ok(match ident.to_string().as_str() {
            "Clone" => Clone,
            "Copy" => Copy,
            "Debug" => Debug,
            "Eq" => Eq,
            "Hash" => Hash,
            "Ord" => Ord,
            "PartialEq" => PartialEq,
            "PartialOrd" => PartialOrd,
            ident => {
                return Err(Error::new(
                    ident.span(),
                    format!("{} isn't supported", ident),
                ))
            }
        })
    }
}

impl Traits {
    fn type_(self) -> Type {
        use Traits::*;

        syn::parse_str(match self {
            Clone => "::core::clone::Clone",
            Copy => "::core::copy::Copy",
            Debug => "::core::fmt::Debug",
            Eq => "::core::cmp::Eq",
            Hash => "::core::hash::Hash",
            Ord => "::core::cmp::Ord",
            PartialEq => "::core::cmp::PartialEq",
            PartialOrd => "::core::cmp::PartialOrd",
        })
        .expect("failed to pass path")
    }

    fn generate_body(self, name: &Ident, data: &Data) -> TokenStream {
        let body = match &data {
            Data::Struct(data) => {
                let pattern = quote! { Self };

                match &data.fields {
                    Fields::Named(fields) => self.generate_struct(name, &pattern, None, fields),
                    Fields::Unnamed(fields) => self.generate_tuple(name, &pattern, None, fields),
                    Fields::Unit => abort_call_site!("Using derive_where on unit struct is not supported as unit structs don't support generics."),
                }
            }
            Data::Enum(data) => {
                let variants: Vec<_> = data.variants.iter().map(|variant| &variant.ident).collect();

                data.variants
                    .iter()
                    .enumerate()
                    .map(|(index, variant)| {
                        let variant_ident = &variant.ident;
                        let variant_fields = &variant.fields;
                        let name = variant_ident;
                        let pattern = quote! { Self::#variant_ident };

                        match variant_fields {
                            Fields::Named(fields) => self.generate_struct(
                                name,
                                &pattern,
                                Some((index, &variants)),
                                fields,
                            ),
                            Fields::Unnamed(fields) => self.generate_tuple(
                                name,
                                &pattern,
                                Some((index, &variants)),
                                fields,
                            ),
                            Fields::Unit => {
                                self.generate_unit(name, &pattern, Some((index, &variants)))
                            }
                        }
                    })
                    .collect()
            }
            Data::Union(_) => todo!("Unions are not supported"),
        };

        self.generate_signature(body)
    }

    fn prepare_ord(
        self,
        fields_temp: &[Ident],
        fields_other: &[Ident],
        variants: Option<(usize, &[&Ident])>,
        skip: &TokenStream,
    ) -> (TokenStream, TokenStream) {
        use Traits::*;

        let type_ = self.type_();

        let mut less = quote! { ::core::cmp::Ordering::Less };
        let mut equal = quote! { ::core::cmp::Ordering::Equal };
        let mut greater = quote! { ::core::cmp::Ordering::Greater };

        match self {
            PartialOrd => {
                less = quote! { ::core::option::Option::Some(#less) };
                equal = quote! { ::core::option::Option::Some(#equal) };
                greater = quote! { ::core::option::Option::Some(#greater) };
            }
            Ord => (),
            _ => unreachable!(),
        };

        let mut body = quote! { #equal };

        for (field_temp, field_other) in fields_temp.iter().zip(fields_other).rev() {
            body = quote! {
                match #type_::partial_cmp(&#field_temp, &#field_other) {
                    #equal => #body,
                    __cmp => __cmp,
                }
            };
        }

        let mut other = quote! {};

        if let Some((variant, variants)) = variants {
            for (index, variants) in variants.iter().enumerate() {
                if variant != index {
                    let ordering = match variant.cmp(&index) {
                        Ordering::Less => &less,
                        Ordering::Equal => &equal,
                        Ordering::Greater => &greater,
                    };

                    other.extend(quote! {
                        Self::#variants #skip => #ordering,
                    })
                }
            }
        }

        (body, other)
    }

    fn generate_signature(self, body: TokenStream) -> TokenStream {
        use Traits::*;

        let type_ = self.type_();

        match self {
            Clone => quote! {
                fn clone(&self) -> Self {
                    match self {
                        #body
                    }
                }
            },
            Copy => quote! {},
            Debug => quote! {
                fn fmt(&self, __f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        #body
                    }
                }
            },
            Eq => quote! {},
            Hash => quote! {
                fn hash<__H: ::core::hash::Hasher>(&self, __state: &mut __H) {
                    #type_::hash(&::core::mem::discriminant(self), __state);

                    match self {
                        #body
                    }
                }
            },
            Ord => quote! {
                fn cmp(&self, __other: &Self) -> ::core::cmp::Ordering {
                    match (self, __other) {
                        #body
                    }
                }
            },
            PartialEq => quote! {
                fn eq(&self, __other: &Self) -> bool {
                    if ::core::mem::discriminant(self) == ::core::mem::discriminant(__other) {
                        let mut __cmp = true;

                        match (self, __other) {
                            #body
                            _ => ::core::unreachable("Comparing discriminants failed")
                        }
                    } else {
                        false
                    }
                }
            },
            PartialOrd => quote! {
                fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                    match self {
                        #body
                    }
                }
            },
        }
    }

    fn generate_struct(
        self,
        name: &Ident,
        pattern: &TokenStream,
        variants: Option<(usize, &[&Ident])>,
        fields: &FieldsNamed,
    ) -> TokenStream {
        use Traits::*;

        let type_ = self.type_();

        let name = name.to_string();

        let fields: Vec<_> = fields
            .named
            .iter()
            .map(|field| field.ident.as_ref().unwrap())
            .collect();

        let fields_temp: Vec<_> = fields
            .iter()
            .map(|field| format_ident!("__{}", field))
            .collect();

        let fields_other: Vec<_> = fields
            .iter()
            .map(|field| format_ident!("__other_{}", field))
            .collect();

        match self {
            Clone => quote! {
                #pattern { #(#fields: #fields_temp),* } => #pattern { #(#fields: #type_::clone(&#fields_temp)),* },
            },
            Copy => quote! {},
            Debug => quote! {
                #pattern { #(#fields: #fields_temp),* } => {
                    let mut __builder = ::core::fmt::Formatter::debug_struct(__f, #name);
                    #(::core::fmt::DebugStruct::field(&mut __builder, #fields, &#fields_temp);)*
                    ::core::fmt::DebugStruct::finish(&mut __builder)
                }
            },
            Eq => quote! {},
            Hash => quote! {
                #pattern { #(#fields: #fields_temp),* } => { #(#type_::hash(&#fields_temp, __state);)* }
            },
            Ord => {
                let (body, other) =
                    self.prepare_ord(&fields_temp, &fields_other, variants, &quote! { { .. } });

                quote! {
                    #pattern { #(#fields: #fields_temp),* } => {
                        match __other {
                            #pattern { #(#fields: #fields_other),* } => #body,
                            #other
                        }
                    }
                }
            }
            PartialEq => quote! {
                (#pattern { #(#fields: #fields_temp),* }, #pattern { #(#fields: #fields_other),* }) => {
                    #(__cmp &= #type_::eq(&#fields_temp, &#fields_other);)*
                }
            },
            PartialOrd => {
                let (body, other) =
                    self.prepare_ord(&fields_temp, &fields_other, variants, &quote! { { .. } });

                quote! {
                    #pattern { #(#fields: #fields_temp),* } => {
                        match __other {
                            #pattern { #(#fields: #fields_other),* } => #body,
                            #other
                        }
                    }
                }
            }
        }
    }

    fn generate_tuple(
        self,
        name: &Ident,
        pattern: &TokenStream,
        variants: Option<(usize, &[&Ident])>,
        fields: &FieldsUnnamed,
    ) -> TokenStream {
        use Traits::*;

        let type_ = self.type_();

        let name = name.to_string();

        let fields_temp: Vec<_> = (0..fields.unnamed.len())
            .into_iter()
            .map(|field| format_ident!("__{}", field))
            .collect();

        let fields_other: Vec<_> = (0..fields.unnamed.len())
            .into_iter()
            .map(|field| format_ident!("__other_{}", field))
            .collect();

        match self {
            Clone => quote! {
                #pattern(#(#fields_temp),*) => #pattern (#(#type_::clone(&#fields_temp)),*),
            },
            Copy => quote! {},
            Debug => quote! {
                #pattern(#(#fields_temp),*) => {
                    let mut __builder = ::core::fmt::Formatter::debug_tuple(__f, #name);
                    #(::core::fmt::DebugTuple::field(&mut __builder, &#fields_temp);)*
                    ::core::fmt::DebugTuple::finish(&mut __builder)
                }
            },
            Eq => quote! {},
            Hash => quote! {
                #pattern(#(#fields_temp),*) => { #(#type_::hash(&#fields_temp, __state);)* }
            },
            Ord => {
                let (body, other) =
                    self.prepare_ord(&fields_temp, &fields_other, variants, &quote! { (..) });

                quote! {
                    #pattern (#(#fields_other),*) => {
                        match __other {
                            #pattern (#(#fields_other),*) => #body,
                            #other
                        }
                    }
                }
            }
            PartialEq => quote! {
                (#pattern(#(#fields_temp),*), #pattern(#(#fields_other),*)) => {
                    #(__cmp &= #type_::eq(&#fields_temp, &#fields_other);)*
                }
            },
            PartialOrd => {
                let (body, other) =
                    self.prepare_ord(&fields_temp, &fields_other, variants, &quote! { (..) });

                quote! {
                    #pattern (#(#fields_other),*) => {
                        match __other {
                            #pattern (#(#fields_other),*) => #body,
                            #other
                        }
                    }
                }
            }
        }
    }

    fn generate_unit(
        self,
        name: &Ident,
        pattern: &TokenStream,
        variants: Option<(usize, &[&Ident])>,
    ) -> TokenStream {
        use Traits::*;

        let name = name.to_string();

        match self {
            Clone => quote! { #pattern => #pattern, },
            Copy => quote! {},
            Debug => quote! { #pattern => ::core::fmt::Formatter::write_str(__f, #name), },
            Eq => quote! {},
            Hash => quote! { #pattern => (), },
            Ord => {
                let (body, other) = self.prepare_ord(&[], &[], variants, &quote! {});

                quote! {
                    #pattern => {
                        match __other {
                            #pattern => #body,
                            #other
                        }
                    }
                }
            }
            PartialEq => quote! { (#pattern, #pattern) => true, },
            PartialOrd => {
                let (body, other) = self.prepare_ord(&[], &[], variants, &quote! {});

                quote! {
                    #pattern => {
                        match __other {
                            #pattern => #body,
                            #other
                        }
                    }
                }
            }
        }
    }
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn derive_where(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let quote_item: TokenStream = item.clone().into();
    let mut output = quote! { #quote_item };

    let derive_where: DeriveWhere = syn::parse(attr).unwrap();

    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = parse_macro_input!(item);

    for trait_ in &derive_where.traits {
        let body = trait_.generate_body(&ident, &data);
        let trait_ = trait_.type_();

        let bounds = if derive_where.bounds.is_empty() {
            quote! {}
        } else {
            let mut bounds = quote! { where };

            for bound in &derive_where.bounds {
                bounds.extend(quote! {
                    #bound: #trait_,
                })
            }

            bounds
        };
        let (impl_generics, type_generics, ..) = generics.split_for_impl();

        output.extend(quote! {
            impl #impl_generics #trait_ for #ident #type_generics
            #bounds
            {
                #body
            }
        })
    }

    output.into()
}
