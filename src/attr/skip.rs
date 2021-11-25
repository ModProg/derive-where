//! Attribute parsing for the `skip` and `skip_inner` options.

use syn::{spanned::Spanned, Meta, NestedMeta, Result};

use crate::{Error, Trait, TraitImpl, SKIP, SKIP_INNER};

/// Stores what [`Trait`]s to skip this field or variant for.
pub enum Skip {
    /// Field skipped for no [`Trait`].
    None,
    /// Field skipped for all [`Trait`]s that support it.
    All,
    /// Field skipped for the [`Trait`]s listed.
    Traits(Vec<Trait>),
}

impl core::default::Default for Skip {
    fn default() -> Self {
        Skip::None
    }
}

impl Skip {
    /// Returns if variant is [`Skip::None`].
    fn is_none(&self) -> bool {
        // MSRV: `matches!` was added in 1.42.0.
        #[allow(clippy::match_like_matches_macro)]
        {
            if let Skip::None = self {
                true
            } else {
                false
            }
        }
    }

    /// Adds a [`Meta`] to this [`Skip`].
    pub fn add_attribute(&mut self, meta: &Meta) -> Result<()> {
        debug_assert!(meta.path().is_ident(SKIP) || meta.path().is_ident(SKIP_INNER));

        match meta {
            Meta::Path(path) => {
                if self.is_none() {
                    *self = Skip::All;
                    Ok(())
                } else {
                    Err(Error::option_duplicate(
                        path.span(),
                        &meta
                            .path()
                            .get_ident()
                            .expect("unexpected skip syntax")
                            .to_string(),
                    ))
                }
            }
            Meta::List(list) => {
                let traits = match self {
                    Skip::None => {
                        *self = Skip::Traits(Vec::new());

                        if let Skip::Traits(traits) = self {
                            traits
                        } else {
                            unreachable!("unexpected variant")
                        }
                    }
                    Skip::All => return Err(Error::option_skip_all(list.span())),
                    Skip::Traits(traits) => traits,
                };

                for nested_meta in &list.nested {
                    if let NestedMeta::Meta(Meta::Path(path)) = nested_meta {
                        let trait_ = Trait::from_path(path)?;

                        if trait_.supports_skip() {
                            if traits.contains(&trait_) {
                                return Err(Error::option_skip_duplicate(
                                    path.span(),
                                    trait_.as_str(),
                                ));
                            } else {
                                traits.push(trait_)
                            }
                        } else {
                            return Err(Error::option_skip_support(path.span(), trait_.as_str()));
                        }
                    } else {
                        return Err(Error::option_syntax(nested_meta.span()));
                    }
                }

                Ok(())
            }
            _ => Err(Error::option_syntax(meta.span())),
        }
    }

    pub fn skip(&self, trait_: &Trait) -> bool {
        match self {
            Skip::None => false,
            Skip::All => trait_.supports_skip(),
            Skip::Traits(traits) => {
                let skip = traits.contains(trait_);
                debug_assert!(!skip || (skip && trait_.supports_skip()));
                skip
            }
        }
    }
}
