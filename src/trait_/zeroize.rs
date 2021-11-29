//! [`Zeroize`](https://docs.rs/zeroize/1.4.3/zeroize/trait.Zeroize.html) implementation.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Lit, Meta, MetaList, NestedMeta, Path, Result};

use crate::{util, DeriveTrait, Error, Impl, TraitImpl};

/// Dummy-struct implement [`Trait`](crate::Trait) for [`Zeroize`](https://docs.rs/zeroize/1.4.3/zeroize/trait.Zeroize.html) .
pub struct Zeroize;

impl TraitImpl for Zeroize {
    fn as_str(&self) -> &'static str {
        "Zeroize"
    }

    fn default_derive_trait(&self) -> DeriveTrait {
        DeriveTrait::Zeroize {
            crate_: None,
            drop: false,
        }
    }

    fn parse_derive_trait(&self, list: MetaList) -> Result<DeriveTrait> {
        let mut crate_ = None;
        let mut drop = false;

        for nested_meta in list.nested {
            match &nested_meta {
                NestedMeta::Meta(Meta::Path(path)) => {
                    if path.is_ident("drop") {
                        if !drop {
                            drop = true;
                        } else {
                            return Err(Error::option_duplicate(path.span(), "drop"));
                        }
                    } else {
                        return Err(Error::option_trait(path.span(), self.as_str()));
                    }
                }
                NestedMeta::Meta(Meta::NameValue(name_value)) => {
                    if name_value.path.is_ident("crate") {
                        if crate_.is_none() {
                            if let Lit::Str(lit_str) = &name_value.lit {
                                match lit_str.parse() {
                                    Ok(path) => {
                                        crate_ = Some(path);
                                    }
                                    Err(error) => return Err(Error::path(lit_str.span(), error)),
                                }
                            } else {
                                return Err(Error::option_syntax(name_value.lit.span()));
                            }
                        } else {
                            return Err(Error::option_duplicate(name_value.span(), "crate"));
                        }
                    } else {
                        return Err(Error::option_trait(name_value.path.span(), self.as_str()));
                    }
                }
                _ => {
                    return Err(Error::option_syntax(nested_meta.span()));
                }
            }
        }

        Ok(DeriveTrait::Zeroize { crate_, drop })
    }

    fn supports_skip(&self) -> bool {
        true
    }

    fn additional_impl(&self, trait_: &DeriveTrait) -> Option<(Path, TokenStream)> {
        if let DeriveTrait::Zeroize { drop: true, .. } = trait_ {
            let path = trait_.path();

            Some((
                util::path(&["core", "ops", "Drop"]),
                quote! {
                    fn drop(&mut self) {
                        #path::zeroize(self);
                    }
                },
            ))
        } else {
            None
        }
    }

    fn build_signature(&self, _impl_: &Impl, body: &TokenStream) -> TokenStream {
        quote! {
            fn zeroize(&mut self) {
                match self {
                    #body
                }
            }
        }
    }
}
