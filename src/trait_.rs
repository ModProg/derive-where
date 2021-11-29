//! Individual implementation for all traits.

mod clone;
mod common_ord;
mod copy;
mod debug;
mod default;
mod eq;
mod hash;
mod ord;
mod partial_eq;
mod partial_ord;
#[cfg(feature = "zeroize")]
mod zeroize;

use proc_macro2::TokenStream;
use syn::{spanned::Spanned, MetaList, Path, Result, TypeParamBound};

use crate::{DeriveTrait, Error, Impl, Item};

/// Type implementing [`TraitImpl`] for every trait.
#[derive(Eq, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub enum Trait {
    /// [`Clone`].
    Clone,
    /// [`Copy`].
    Copy,
    /// [`Debug`](core::fmt::Debug).
    Debug,
    /// [`Default`].
    Default,
    /// [`Eq`].
    Eq,
    /// [`Hash`](core::hash::Hash).
    Hash,
    /// [`Ord`].
    Ord,
    /// [`PartialEq`].
    PartialEq,
    /// [`PartialOrd`].
    PartialOrd,
    /// [`Zeroize`](https://docs.rs/zeroize/1.4.3/zeroize/trait.Zeroize.html).
    #[cfg(feature = "zeroize")]
    Zeroize,
}

impl Trait {
    /// Return dummy-struct for the internal implementation.
    fn implementation(&self) -> &dyn TraitImpl {
        match self {
            Trait::Clone => &clone::Clone,
            Trait::Copy => &copy::Copy,
            Trait::Debug => &debug::Debug,
            Trait::Default => &default::Default,
            Trait::Eq => &eq::Eq,
            Trait::Hash => &hash::Hash,
            Trait::Ord => &ord::Ord,
            Trait::PartialEq => &partial_eq::PartialEq,
            Trait::PartialOrd => &partial_ord::PartialOrd,
            #[cfg(feature = "zeroize")]
            Trait::Zeroize => &zeroize::Zeroize,
        }
    }

    /// Create [`Trait`] from [`Path`].
    pub fn from_path(path: &Path) -> Result<Self> {
        if let Some(ident) = path.get_ident() {
            use Trait::*;

            match ident.to_string().as_str() {
                "Clone" => Ok(Clone),
                "Copy" => Ok(Copy),
                "Debug" => Ok(Debug),
                "Default" => Ok(Default),
                "Eq" => Ok(Eq),
                "Hash" => Ok(Hash),
                "Ord" => Ok(Ord),
                "PartialEq" => Ok(PartialEq),
                "PartialOrd" => Ok(PartialOrd),
                #[cfg(feature = "zeroize")]
                "Zeroize" => Ok(Zeroize),
                _ => Err(Error::trait_(path.span())),
            }
        } else {
            Err(Error::trait_(path.span()))
        }
    }
}

impl TraitImpl for Trait {
    fn as_str(&self) -> &'static str {
        self.implementation().as_str()
    }

    fn default_derive_trait(&self) -> DeriveTrait {
        self.implementation().default_derive_trait()
    }

    fn parse_derive_trait(&self, list: MetaList) -> Result<DeriveTrait> {
        self.implementation().parse_derive_trait(list)
    }

    fn supports_skip(&self) -> bool {
        self.implementation().supports_skip()
    }

    fn additional_where_bounds(&self, data: &Item) -> Option<TypeParamBound> {
        self.implementation().additional_where_bounds(data)
    }

    fn additional_impl(&self, trait_: &DeriveTrait) -> Option<(Path, TokenStream)> {
        self.implementation().additional_impl(trait_)
    }

    fn build_signature(&self, impl_: &Impl, body: &TokenStream) -> TokenStream {
        self.implementation().build_signature(impl_, body)
    }
}

/// Single trait implementation. Parses attributes and constructs `impl`s.
pub trait TraitImpl {
    /// [`str`] representation of this [`Trait`].
    /// Used to compare against [`Ident`](struct@syn::Ident)s and create error messages.
    fn as_str(&self) -> &'static str;

    /// Associated [`DeriveTrait`].
    fn default_derive_trait(&self) -> DeriveTrait;

    /// Parse a `derive_where` trait with it's options.
    fn parse_derive_trait(&self, list: MetaList) -> Result<DeriveTrait> {
        Err(Error::options(list.span(), self.as_str()))
    }

    /// Returns if [`Trait`] supports skipping fields.
    fn supports_skip(&self) -> bool {
        false
    }

    /// Additional bounds to add to [`WhereClause`](syn::WhereClause).
    fn additional_where_bounds(&self, _data: &Item) -> Option<TypeParamBound> {
        None
    }

    /// Additional implementation to add for this [`Trait`].
    fn additional_impl(&self, _trait_: &DeriveTrait) -> Option<(Path, TokenStream)> {
        None
    }

    /// Build method signature for this [`Trait`].
    fn build_signature(&self, _impl_: &Impl, _body: &TokenStream) -> TokenStream {
        TokenStream::new()
    }
}