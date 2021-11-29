//! Attribute parsing for variants.

use syn::{spanned::Spanned, Attribute, Meta, NestedMeta, Result};

use crate::{DeriveWhere, Error, DERIVE_WHERE};

use super::{Default, Skip};

/// Attributes on variant.
#[derive(Default)]
pub struct VariantAttr {
    /// Default variant.
    pub default: Default,
    /// [`Trait`](crate::Trait)s to skip all fields for.
    pub skip_inner: Skip,
}

impl VariantAttr {
    /// Create [`VariantAttr`] from [`Attribute`]s.
    pub fn from_attrs(
        attrs: &[Attribute],
        derive_wheres: &[DeriveWhere],
        accumulated_defaults: &mut Default,
    ) -> Result<Self> {
        let mut self_ = VariantAttr::default();

        for attr in attrs {
            if attr.path.is_ident(DERIVE_WHERE) {
                match attr.parse_meta() {
                    Ok(meta) => self_.add_meta(&meta, derive_wheres, accumulated_defaults)?,
                    Err(error) => return Err(Error::attribute_syntax(attr.span(), error)),
                }
            }
        }

        Ok(self_)
    }

    /// Add [`Meta`] to [`VariantAttr`].
    fn add_meta(
        &mut self,
        meta: &Meta,
        derive_wheres: &[DeriveWhere],
        accumulated_defaults: &mut Default,
    ) -> Result<()> {
        debug_assert!(meta.path().is_ident(DERIVE_WHERE));

        if let Meta::List(list) = meta {
            for nested_meta in &list.nested {
                match nested_meta {
                    NestedMeta::Meta(meta) => {
                        if meta.path().is_ident(Skip::SKIP_INNER) {
                            self.skip_inner.add_attribute(meta)?;
                        } else if meta.path().is_ident(Default::DEFAULT) {
                            self.default.add_attribute(
                                meta,
                                derive_wheres,
                                accumulated_defaults,
                            )?;
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
}
