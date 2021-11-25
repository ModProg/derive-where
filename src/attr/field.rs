//! Attribute parsing for fields.

use syn::{spanned::Spanned, Attribute, Meta, NestedMeta, Result};

use crate::{trait_::TraitImpl, Error, Trait, DERIVE_WHERE, SKIP};

use super::{Skip, ZeroizeFqs};

/// Attributes on field.
#[derive(Default)]
pub struct FieldAttr {
    /// [`Trait`]s to skip this field for.
    skip: Skip,
    /// Use fully-qualified-syntax for the [`Zeroize`](https://docs.rs/zeroize/1.4.3/zeroize/trait.Zeroize.html) implementation on this field.
    #[cfg(feature = "zeroize")]
    zeroize_fqs: ZeroizeFqs,
}

impl FieldAttr {
    /// Create [`FieldAttr`] from [`Attribute`]s.
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut self_ = FieldAttr::default();

        for attr in attrs {
            if attr.path.is_ident(DERIVE_WHERE) {
                match attr.parse_meta() {
                    Ok(meta) => self_.add_meta(&meta)?,
                    Err(error) => return Err(Error::attribute_syntax(attr.span(), error)),
                }
            }
        }

        Ok(self_)
    }

    /// Add [`Meta`] to [`FieldAttr`].
    fn add_meta(&mut self, meta: &Meta) -> Result<()> {
        debug_assert!(meta.path().is_ident(DERIVE_WHERE));

        if let Meta::List(list) = meta {
            for nested_meta in &list.nested {
                match nested_meta {
                    NestedMeta::Meta(meta) => {
                        if meta.path().is_ident(SKIP) {
                            self.skip.add_attribute(meta)?;
                            continue;
                        }

                        #[cfg(feature = "zeroize")]
                        {
                            if meta.path().is_ident(Trait::Zeroize.as_str()) {
                                self.zeroize_fqs.add_attribute(meta)?;
                                continue;
                            }
                        }

                        return Err(Error::option(meta.path().span()));
                    }
                    _ => return Err(Error::option_syntax(nested_meta.span())),
                }
            }

            Ok(())
        } else {
            Err(Error::option_syntax(meta.span()))
        }
    }
}
