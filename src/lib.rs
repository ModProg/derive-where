use proc_macro2::{Ident, TokenStream};
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Data, DeriveInput, Error, Fields, FieldsNamed, FieldsUnnamed, Token, Type,
};

#[derive(Debug)]
struct DeriveWhere {
    bounds: Vec<Ident>,
    traits: Vec<Traits>,
}

impl Parse for DeriveWhere {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let bounds = Punctuated::<Ident, Token![,]>::parse_separated_nonempty(input)?
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
    Debug,
    Eq,
    Hash,
    PartialEq,
    PartialOrd,
    Ord,
}

impl Parse for Traits {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        use Traits::*;
        let ident = Ident::parse(input)?;

        Ok(match ident.to_string().as_str() {
            "Clone" => Clone,
            "Debug" => Debug,
            "Eq" => Eq,
            "Hash" => Hash,
            "PartialEq" => PartialEq,
            "PartialOrd" => PartialOrd,
            "Ord" => Ord,
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
            Debug => "::core::fmt::Debug",
            Eq => "::core::cmp::Eq",
            Hash => "::core::hash::Hash",
            PartialEq => "::core::cmp::PartialEq",
            PartialOrd => "::core::cmp::PartialOrd",
            Ord => "::core::cmp::Ord",
        })
        .expect("failed to pass path")
    }

    fn generate_body(self, name: &str, data: &Data) -> TokenStream {
        let body = match &data {
            Data::Struct(data) => {
                let variant = quote! { Self };

                match &data.fields {
                    Fields::Named(fields) => self.generate_struct(name, &variant, fields),
                    Fields::Unnamed(fields) => self.generate_tuple(name, &variant, fields),
                    Fields::Unit => abort_call_site!("Using derive_where on unit struct is not supported as unit structs don't support generics.")
                }
            }
            Data::Enum(data) => data
                .variants
                .iter()
                .map(|variant| {
                    let variant_ident = &variant.ident;
                    let variant_fields = &variant.fields;
                    let name = variant_ident.to_string();
                    let variant = quote! { Self::#variant_ident };

                    match variant_fields {
                        Fields::Named(fields) => self.generate_struct(&name, &variant, fields),
                        Fields::Unnamed(fields) => self.generate_tuple(&name, &variant, fields),
                        Fields::Unit => self.generate_unit(&name, &variant),
                    }
                })
                .collect(),
            Data::Union(_) => abort_call_site!("Using derive_where on Unions is not supported."),
        };

        self.generate_signature(body)
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
                    match (self, __other) {
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
        }
    }

    fn generate_struct(
        self,
        name: &str,
        variant: &TokenStream,
        fields: &FieldsNamed,
    ) -> TokenStream {
        use Traits::*;

        let type_ = self.type_();

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
                #variant { #(#fields: #fields_temp),* } => #variant { #(#fields: #type_::clone(&#fields_temp)),* },
            },
            Debug => quote! {
                #variant { #(#fields: #fields_temp),* } => {
                    let __builder = ::core::fmt::Formatter::debug_struct(__f, #name);
                    #(::core::fmt::DebugStruct::field(__builder, #fields, &#fields_temp);)*
                    ::core::fmt::DebugStruct::finish(__builder)
                }
            },
            Eq => quote! {},
            Hash => quote! {
                #variant { #(#fields: #fields_temp),* } => { #(#type_::hash(&#fields_temp, __state);)* }
            },
            PartialEq => quote! {
                (#variant { #(#fields: #fields_temp),* }, #variant { #(#fields: #fields_other),* }) => {
                    #(__cmp &= #type_::eq(&#fields_temp, &#fields_other);)*
                }
            },
            PartialOrd => todo!(),
            Ord => todo!(),
        }
    }

    fn generate_tuple(
        self,
        name: &str,
        variant: &TokenStream,
        fields: &FieldsUnnamed,
    ) -> TokenStream {
        use Traits::*;

        let type_ = self.type_();

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
                #variant(#(#fields_temp),*) => #variant (#(#type_::clone(&#fields_temp)),*),
            },
            Debug => quote! {
                #variant(#(#fields_temp),*) => {
                    let __builder = ::core::fmt::Formatter::tuple(__f, #name);
                    #(::core::fmt::DebugTuple::field(__builder, &#fields_temp);)*
                    ::core::fmt::DebugTuple::finish(__builder)
                }
            },
            Eq => quote! {},
            Hash => quote! {
                #variant(#(#fields_temp),*) => { #(#type_::hash(&#fields_temp, __state);)* }
            },
            PartialEq => quote! {
                (#variant(#(#fields_temp),*), #variant(#(#fields_other),*)) => {
                    #(__cmp &= #type_::eq(&#fields_temp, &#fields_other);)*
                }
            },
            PartialOrd => todo!(),
            Ord => todo!(),
        }
    }

    fn generate_unit(self, name: &str, variant: &TokenStream) -> TokenStream {
        use Traits::*;

        match self {
            Clone => quote! { #variant => #variant, },
            Debug => quote! { ::core::fmt::Formatter::write_str(__f, #name), },
            Eq => quote! {},
            Hash => quote! { #variant => (), },
            PartialEq => quote! {
                (#variant, #variant) => true,
            },
            PartialOrd => todo!(),
            Ord => todo!(),
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
        let body = trait_.generate_body(&ident.to_string(), &data);
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

        output.extend(quote! {
            impl #generics #trait_ for #ident #generics
            #bounds
            {
                #body
            }
        })
    }

    output.into()
}
