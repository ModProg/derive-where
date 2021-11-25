//! Attribute parsing for variants.

use syn::{spanned::Spanned, Attribute, Meta, NestedMeta, Result};

use crate::{Error, Trait, DEFAULT, DERIVE_WHERE, SKIP_INNER};

use super::{Default, Skip};

/// Attributes on variant.
#[derive(Default)]
pub struct VariantAttr {
    /// Default variant.
    default: Default,
    /// [`Trait`]s to skip all fields for.
    skip_inner: Skip,
}

impl VariantAttr {
    /// Create [`VariantAttr`] from [`Attribute`]s.
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut self_ = VariantAttr::default();

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

    /// Add [`Meta`] to [`VariantAttr`].
    fn add_meta(&mut self, meta: &Meta) -> Result<()> {
        debug_assert!(meta.path().is_ident(DERIVE_WHERE));

        if let Meta::List(list) = meta {
            for nested_meta in &list.nested {
                match nested_meta {
                    NestedMeta::Meta(meta) => {
                        if meta.path().is_ident(SKIP_INNER) {
                            self.skip_inner.add_attribute(meta)?;
                        } else if meta.path().is_ident(DEFAULT) {
                            self.default.add_attribute(meta)?;
                        } else {
                            return Err(Error::option(meta.path().span()));
                        }
                    }
                    _ => return Err(Error::option_syntax(nested_meta.span())),
                }
            }

            Ok(())
        } else {
            Err(Error::option_syntax(meta.span()))
        }
    }

    /// Returns `true` if this variant is skipped with the given [`Trait`].
    pub fn skip(&self, trait_: &Trait) -> bool {
        self.skip_inner.skip(trait_)
    }
}
