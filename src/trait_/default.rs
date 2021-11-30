//! [`Default`](core::default::Default) implementation.

use proc_macro2::TokenStream;
use quote::quote;

use crate::{Data, DeriveTrait, Impl, SimpleType, TraitImpl};

/// Dummy-struct implement [`Trait`](crate::Trait) for [`Default`](core::default::Default).
pub struct Default;

impl TraitImpl for Default {
    fn as_str(&self) -> &'static str {
        "Default"
    }

    fn default_derive_trait(&self) -> DeriveTrait {
        DeriveTrait::Default
    }

    fn build_signature(&self, _impl_: &Impl, body: &TokenStream) -> TokenStream {
        quote! {
            fn default() -> Self {
                #body
            }
        }
    }

    fn build_body(&self, trait_: &DeriveTrait, data: &Data) -> TokenStream {
        if data.is_default() {
            let path = &data.path;

            match data.simple_type() {
                SimpleType::Struct(fields) => {
                    let fields = fields.iter_field_ident(trait_);
                    let trait_path = trait_.path();

                    quote! { #path { #(#fields: #trait_path::default()),* } }
                }
                SimpleType::Tuple(fields) => {
                    let trait_path = trait_.path();
                    let fields = fields
                        .iter_fields(trait_)
                        .map(|_| quote! { #trait_path::default() });

                    quote! { #path(#(#fields),*) }
                }
                SimpleType::Unit(_) => {
                    quote! { #path }
                }
                SimpleType::Union(_) => unreachable!("unexpected trait for union"),
            }
        }
        // Skip `Default` implementation if variant isn't marked with a `default` attribute.
        else {
            TokenStream::new()
        }
    }
}
