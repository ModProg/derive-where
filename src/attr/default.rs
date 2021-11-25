//! Attribute parsing for the `default` option.

use syn::{spanned::Spanned, Meta, Result};

use crate::{Error, DEFAULT};

/// Stores if this variant should be the default when implementing [`Default`](core::default::Default).
#[derive(Default)]
pub struct Default(bool);

impl Default {
    /// Adds a [`Meta`] to this [`Default`].
    pub fn add_attribute(&mut self, meta: &Meta) -> Result<()> {
        debug_assert!(meta.path().is_ident(DEFAULT));

        match meta {
            Meta::Path(path) => {
                if self.0 {
                    Err(Error::option_duplicate(path.span(), DEFAULT))
                } else {
                    self.0 = true;
                    Ok(())
                }
            }
            _ => Err(Error::option_syntax(meta.span())),
        }
    }
}
