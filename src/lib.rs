use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields, FieldsNamed, FieldsUnnamed, Type};

#[derive(Debug)]
struct DeriveWhere {
    bounds: Vec<Ident>,
    traits: Vec<Traits>,
}

impl Parse for DeriveWhere {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut bounds_done = false;
        let mut bounds = Vec::new();
        let mut traits = Vec::new();

        input.step(|cursor| {
            let mut rest = *cursor;

            while let Some((tt, next)) = rest.token_tree() {
                rest = next;

                if bounds_done {
                    if let TokenTree::Ident(ident) = tt {
                        traits.push(ident.try_into()?)
                    } else {
                        return Err(Error::new(tt.span(), format!("Unexpected token: {}", tt)));
                    }
                } else {
                    match tt {
                        TokenTree::Punct(punct) if punct.as_char() == ';' => bounds_done = true,
                        TokenTree::Ident(ident) => {
                            // TODO: check if these are really `syn::TraitBound`s
                            bounds.push(ident)
                        }
                        // TODO: check correct usage of comma
                        TokenTree::Punct(punct) if punct.as_char() == ',' => (),
                        tt => {
                            return Err(Error::new(
                                tt.span(),
                                format!("Unexpected token: `{}`", tt),
                            ))
                        }
                    }
                }
            }

            Ok(((), rest))
        })?;

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

impl TryFrom<Ident> for Traits {
    type Error = Error;

    fn try_from(ident: Ident) -> Result<Self, Self::Error> {
        use Traits::*;

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
        .expect("couldn't pass path to trait")
    }

    fn generate_body(self, name: &str, data: &Data) -> TokenStream {
        let body = match &data {
            Data::Struct(data) => {
                let variant = quote! { Self };

                match &data.fields {
                    Fields::Named(fields) => self.generate_struct(name, &variant, fields),
                    Fields::Unnamed(fields) => self.generate_tuple(name, &variant, fields),
                    Fields::Unit => self.generate_unit(name, &variant),
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
            Data::Union(_) => todo!("Unions are not supported"),
        };

        self.generate_signature(body)
    }

    fn generate_signature(self, body: TokenStream) -> TokenStream {
        use Traits::*;

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
                    match self {
                        #body
                    }
                }
            },
            PartialEq => quote! {
                fn eq(&self, __other: &Self) -> bool {
                    let mut __cmp = true;

                    match (self, __other) {
                        #body
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
            Ord => todo!(),
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

        match self {
            Clone => quote! {
                #variant { #(#fields: #fields_temp),* } => { #variant { #(#fields: #type_::clone(&#fields_temp)),* } }
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
                (#variant { #(#fields: #fields_temp),* }, #variant { #(#fields: #fields_temp),* }) => {
                    #(__cmp &= #type_::eq(&#fields_temp, &#fields_temp);)*
                }
            },
            PartialOrd => todo!(),
            Ord => todo!(),
        }
    }

    fn generate_tuple(
        self,
        _name: &str,
        variant: &TokenStream,
        fields: &FieldsUnnamed,
    ) -> TokenStream {
        use Traits::*;

        let type_ = self.type_();

        let fields_temp: Vec<_> = (0..fields.unnamed.len())
            .into_iter()
            .map(|field| format_ident!("__{}", field))
            .collect();

        match self {
            Clone => quote! {
                #variant(#(#fields_temp),*) => #variant (#(#type_::clone(&#fields_temp)),*),
            },
            Debug => todo!(),
            Eq => todo!(),
            Hash => todo!(),
            PartialEq => todo!(),
            PartialOrd => todo!(),
            Ord => todo!(),
        }
    }

    fn generate_unit(self, _name: &str, variant: &TokenStream) -> TokenStream {
        use Traits::*;

        match self {
            Clone => quote! { #variant => #variant, },
            Debug => todo!(),
            Eq => todo!(),
            Hash => todo!(),
            PartialEq => todo!(),
            PartialOrd => todo!(),
            Ord => todo!(),
        }
    }
}

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
