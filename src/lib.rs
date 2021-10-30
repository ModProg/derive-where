use proc_macro::{self, TokenStream, TokenTree};
use quote::__private::TokenStream as TS;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DataEnum, DataStruct, DeriveInput};

enum Traits {
    Clone,
}

impl TryFrom<TokenStream> for Traits {
    type Error = String;

    fn try_from(value: TokenStream) -> Result<Self, Self::Error> {
        use Traits::*;
        if let Some(TokenTree::Ident(i)) = value.into_iter().next() {
            Ok(match i.to_string().as_str() {
                "Clone" => Clone,
                _ => return Err(format!("")),
            })
        } else {
            Err(format!(""))
        }
    }
}

impl Traits {
    fn ident(&self) -> syn::Ident {
        use Traits::*;

        format_ident!(
            "{}",
            match self {
                Clone => "Clone",
            }
        )
    }

    fn body_struct(&self, data: DataStruct) -> TS {
        use Traits::*;
        match self {
            Clone => {
                let body = match data.fields {
                    syn::Fields::Named(f) => {
                        let fields: Vec<_> = f
                            .named
                            .into_iter()
                            .map(|f| f.ident.expect("Every field should have a name"))
                            .collect();
                        quote! {
                            match self {
                                Self{#(#fields),*} => Self{#(#fields: #fields.clone()),*}
                            }
                        }
                    }
                    syn::Fields::Unnamed(f) => {
                        let fields: Vec<_> = (0..f.unnamed.len())
                            .into_iter()
                            .map(|n| format_ident!("field{}", n))
                            .collect();
                        quote! {
                            match self {
                                Self(#(#fields),*) => Self(#(#fields.clone()),*)
                            }
                        }
                    }
                    syn::Fields::Unit => quote! {Self},
                };
                quote! {
                    fn clone(&self) -> Self {
                        #body
                    }
                }
            }
        }
    }
    fn body_enum(&self, data: DataEnum) -> TS {
        use Traits::*;
        match self {
            Clone => {
                let body: Vec<_> = data
                    .variants
                    .iter()
                    .map(|v| {
                        let ident = &v.ident;
                        match &v.fields {
                            syn::Fields::Named(f) => {
                                let fields: Vec<_> = f
                                    .named.clone()
                                    .into_iter()
                                    .map(|f| f.ident.expect("Every field should have a name"))
                                    .collect();
                                quote! {
                                    Self::#ident{#(#fields),*} => Self::#ident{#(#fields: #fields.clone()),*}
                                }
                            }
                            syn::Fields::Unnamed(f) => {
                                let fields: Vec<_> = (0..f.unnamed.len())
                                    .into_iter()
                                    .map(|n| format_ident!("field{}", n))
                                    .collect();
                                quote! {
                                    Self::#ident(#(#fields),*) => Self::#ident(#(#fields.clone()),*)
                                }
                            }
                            syn::Fields::Unit => quote! {Self::#ident => Self::#ident},
                        }
                    })
                    .collect();
                quote! {
                    fn clone(&self) -> Self {
                        match self {
                            #(#body),*
                        }
                    }
                }
            }
        }
    }
}

#[proc_macro_attribute]
pub fn derive_where(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item2: TS = item.clone().into();
    let mut iter = attr.into_iter();
    let w: TokenStream = iter
        .by_ref()
        .take_while(|a| !matches!(a, proc_macro::TokenTree::Punct(p) if p.as_char() == ';'))
        .collect();
    let w: TS = w.into();
    let t: TokenStream = iter.collect();
    let t: Traits = t.try_into().unwrap();

    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = parse_macro_input!(item);

    let body = match data {
        syn::Data::Struct(s) => t.body_struct(s),
        syn::Data::Enum(e) => t.body_enum(e),
        syn::Data::Union(_) => todo!("Unions are not supported"),
    };

    let t = t.ident();
    let output = quote! {
        #item2
        impl #generics #t for #ident #generics
            where #w
        {
            #body
        }
    };
    output.into()
}
