use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields, Type};

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

#[derive(Debug)]
enum Traits {
    Clone,
    /*Debug,
    Eq,
    Hash,
    PartialEq,
    PartialOrd,
    Ord,*/
}

impl TryFrom<Ident> for Traits {
    type Error = Error;

    fn try_from(ident: Ident) -> Result<Self, Self::Error> {
        use Traits::*;

        Ok(match ident.to_string().as_str() {
            "Clone" => Clone,
            /*"Debug" => Debug,
            "Eq" => Eq,
            "Hash" => Hash,
            "PartialEq" => PartialEq,
            "PartialOrd" => PartialOrd,
            "Ord" => Ord,*/
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
    fn type_(&self) -> Type {
        use Traits::*;

        syn::parse_str(match self {
            Clone => "::core::clone::Clone",
            /*Debug => "::core::fmt::Debug",
            Eq => "::core::cmp::Eq",
            Hash => "::core::hash::Hash",
            PartialEq => "::core::cmp::PartialEq",
            PartialOrd => "::core::cmp::PartialOrd",
            Ord => "::core::cmp::Ord",*/
        })
        .expect("couldn't pass path to trait")
    }

    fn generate_body(&self, data: &Data) -> TokenStream {
        let body = match &data {
            Data::Struct(data) => {
                let name = quote! { Self };

                match &data.fields {
                    Fields::Named(fields) => {
                        let fields: Vec<_> = fields
                            .named
                            .iter()
                            .map(|f| f.ident.as_ref().expect("Every field should have a name"))
                            .collect();

                        let fields_temp: Vec<_> = fields
                            .iter()
                            .map(|field| format_ident!("__{}", field))
                            .collect();

                        let fields_destructure: Vec<_> = fields
                            .iter()
                            .zip(&fields_temp)
                            .map(|(field, field_temp)| quote! { #field: #field_temp })
                            .collect();

                        let body = self.generate_struct(name, fields, fields_temp);

                        quote! {
                            let Self { #(#fields_destructure),* } = self;
                            #body
                        }
                    }
                    Fields::Unnamed(fields) => {
                        let fields_temp: Vec<_> = (0..fields.unnamed.len())
                            .into_iter()
                            .map(|field| format_ident!("__{}", field))
                            .collect();

                        let body = self.generate_tuple(name, &fields_temp);

                        quote! {
                            let Self ( #(#fields_temp),* ) = self;
                            #body
                        }
                    }
                    Fields::Unit => self.generate_unit(name),
                }
            }
            Data::Enum(data) => {
                let bodies: Vec<_> = data
                    .variants
                    .iter()
                    .map(|variant| {
                        let variant_ident = &variant.ident;
                        let name = quote! { Self::#variant_ident };

                        match &variant.fields {
                            Fields::Named(fields) => {
                                let fields: Vec<_> = fields
                                    .named
                                    .iter()
                                    .map(|f| {
                                        f.ident.as_ref().expect("Every field should have a name")
                                    })
                                    .collect();

                                let fields_temp: Vec<_> = fields
                                    .iter()
                                    .map(|field| format_ident!("__{}", field))
                                    .collect();

                                let fields_destructure: Vec<_> = fields
                                    .iter()
                                    .zip(&fields_temp)
                                    .map(|(field, field_temp)| quote! { #field: #field_temp })
                                    .collect();

                                let body = self.generate_struct(name, fields, fields_temp);

                                quote! {
                                    Self::#variant_ident { #(#fields_destructure),* } => { #body }
                                }
                            }
                            Fields::Unnamed(fields) => {
                                let fields_temp: Vec<_> = (0..fields.unnamed.len())
                                    .into_iter()
                                    .map(|field| format_ident!("__{}", field))
                                    .collect();

                                let body = self.generate_tuple(name, &fields_temp);

                                quote! {
                                    Self::#variant_ident ( #(#fields_temp),* ) => { #body }
                                }
                            }
                            Fields::Unit => {
                                let body = self.generate_unit(name);
                                quote! { Self::#variant_ident => { #body } }
                            }
                        }
                    })
                    .collect();

                quote! {
                    match self {
                        #(#bodies),*
                    }
                }
            }
            Data::Union(_) => todo!("Unions are not supported"),
        };

        self.generate_signature(body)
    }

    fn generate_signature(&self, body: TokenStream) -> TokenStream {
        use Traits::*;

        match self {
            Clone => quote! {
                fn clone(&self) -> Self {
                    #body
                }
            },
        }
    }

    fn generate_struct(
        &self,
        name: TokenStream,
        fields: Vec<&Ident>,
        fields_temp: Vec<Ident>,
    ) -> TokenStream {
        use Traits::*;

        let type_ = self.type_();

        match self {
            Clone => {
                let assigns = fields
                    .into_iter()
                    .zip(fields_temp)
                    .map(|(field, field_temp)| {
                        quote! { #field: #type_::clone(&#field_temp) }
                    });

                quote! {
                    #name { #(#assigns),* }
                }
            }
        }
    }

    fn generate_tuple(&self, name: TokenStream, fields_temp: &[Ident]) -> TokenStream {
        use Traits::*;

        let type_ = self.type_();

        match self {
            Clone => {
                let assigns = fields_temp.iter().map(|field_temp| {
                    quote! { #type_::clone(&#field_temp) }
                });

                quote! {
                    #name (#(#assigns),*)
                }
            }
        }
    }

    fn generate_unit(&self, name: TokenStream) -> TokenStream {
        use Traits::*;

        match self {
            Clone => quote! { #name },
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
        let body = trait_.generate_body(&data);
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
