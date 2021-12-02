//! Attribute parsing for variants.

use syn::{spanned::Spanned, Attribute, Fields, Meta, NestedMeta, Result, Variant};

use crate::{Default, DeriveWhere, Error, Skip, DERIVE_WHERE};

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
		variant: &Variant,
	) -> Result<Self> {
		let mut self_ = VariantAttr::default();

		for attr in attrs {
			if attr.path.is_ident(DERIVE_WHERE) {
				match attr.parse_meta() {
					Ok(meta) => {
						self_.add_meta(&meta, derive_wheres, accumulated_defaults, variant)?
					}
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
		variant: &Variant,
	) -> Result<()> {
		debug_assert!(meta.path().is_ident(DERIVE_WHERE));

		if let Meta::List(list) = meta {
			if list.nested.is_empty() {
				return Err(Error::empty(list.span()));
			}

			for nested_meta in &list.nested {
				match nested_meta {
					NestedMeta::Meta(meta) => {
						if meta.path().is_ident(Skip::SKIP_INNER) {
							// Don't allow `skip_inner` on empty variants.
							match &variant.fields {
								Fields::Named(fields) if fields.named.is_empty() => {
									return Err(Error::option_skip_empty(variant.span()))
								}
								Fields::Unnamed(fields) if fields.unnamed.is_empty() => {
									return Err(Error::option_skip_empty(variant.span()))
								}
								Fields::Unit => {
									return Err(Error::option_skip_empty(variant.span()))
								}
								_ => self.skip_inner.add_attribute(derive_wheres, None, meta)?,
							}
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
