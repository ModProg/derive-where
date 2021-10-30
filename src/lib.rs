use proc_macro2::{TokenStream, TokenTree};
use quote::{format_ident, quote};
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput};

#[derive(Debug)]
struct DeriveWhere {
    bounds: Vec<proc_macro2::Ident>,
    traits: Vec<Traits>,
}

impl Parse for DeriveWhere {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut bounds_done = false;
        let mut bounds = Vec::new();
        let mut traits = Vec::<Traits>::new();

        input.step(|cursor| {
            let mut rest = *cursor;

            while let Some((tt, next)) = rest.token_tree() {
                rest = next;

                if bounds_done {
                    if let TokenTree::Ident(ident) = tt {
                        traits.push(ident.try_into()?)
                    } else {
                        return Err(syn::Error::new(
                            tt.span(),
                            format!("Unexpected token: {}", tt),
                        ));
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
                            return Err(syn::Error::new(
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

impl TryFrom<proc_macro2::Ident> for Traits {
    type Error = syn::Error;

    fn try_from(ident: proc_macro2::Ident) -> Result<Self, Self::Error> {
        Ok(match ident.to_string().as_str() {
            "Clone" => Self::Clone,
            /*"Debug" => Self::Debug,
            "Eq" => Self::Eq,
            "Hash" => Self::Hash,
            "PartialEq" => Self::PartialEq,
            "PartialOrd" => Self::PartialOrd,
            "Ord" => Self::Ord,*/
            ident => {
                return Err(syn::Error::new(
                    ident.span(),
                    format!("{} isn't supported", ident),
                ))
            }
        })
    }
}

impl Traits {
    fn type_(&self) -> syn::Type {
        syn::parse_str(match self {
            Self::Clone => "::core::clone::Clone",
            /*Self::Debug => "::core::fmt::Debug",
            Self::Eq => "::core::cmp::Eq",
            Self::Hash => "::core::hash::Hash",
            Self::PartialEq => "::core::cmp::PartialEq",
            Self::PartialOrd => "::core::cmp::PartialOrd",
            Self::Ord => "::core::cmp::Ord",*/
        })
        .expect("couldn't pass path to trait")
    }

    fn generate_body(&self, data: &syn::Data) -> TokenStream {
        let body = match &data {
            syn::Data::Struct(data) => {
                let name = quote! { Self };

                match &data.fields {
                    syn::Fields::Named(fields) => {
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
                    syn::Fields::Unnamed(fields) => {
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
                    syn::Fields::Unit => self.generate_unit(name),
                }
            }
            syn::Data::Enum(data) => {
                let bodies: Vec<_> = data
                    .variants
                    .iter()
                    .map(|variant| {
                        let variant_ident = &variant.ident;
                        let name = quote! { Self::#variant_ident };

                        match &variant.fields {
                            syn::Fields::Named(fields) => {
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
                            syn::Fields::Unnamed(fields) => {
                                let fields_temp: Vec<_> = (0..fields.unnamed.len())
                                    .into_iter()
                                    .map(|field| format_ident!("__{}", field))
                                    .collect();

                                let body = self.generate_tuple(name, &fields_temp);

                                quote! {
                                    Self::#variant_ident ( #(#fields_temp),* ) => { #body }
                                }
                            }
                            syn::Fields::Unit => {
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
            syn::Data::Union(_) => todo!("Unions are not supported"),
        };

        self.generate_signature(body)
    }

    fn generate_signature(&self, body: TokenStream) -> TokenStream {
        match self {
            Traits::Clone => quote! {
                fn clone(&self) -> Self {
                    #body
                }
            },
        }
    }

    fn generate_struct(
        &self,
        name: TokenStream,
        fields: Vec<&proc_macro2::Ident>,
        fields_temp: Vec<proc_macro2::Ident>,
    ) -> TokenStream {
        let type_ = self.type_();

        match self {
            Traits::Clone => {
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

    fn generate_tuple(&self, name: TokenStream, fields_temp: &[proc_macro2::Ident]) -> TokenStream {
        let type_ = self.type_();

        match self {
            Traits::Clone => {
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
        match self {
            Traits::Clone => quote! { #name },
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
