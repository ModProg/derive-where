//! Utility functions.

use core::iter::FromIterator;

use proc_macro2::Span;
use syn::{punctuated::Punctuated, Ident, Path, PathArguments, PathSegment, Token};

use crate::Item;

/// Create [`PathSegment`] from [`str`].
pub fn path_segment(ident: &str) -> PathSegment {
    PathSegment {
        ident: Ident::new(ident, Span::call_site()),
        arguments: PathArguments::None,
    }
}

/// Create [`Path`] from `[&str]`.
pub fn path(segments: &[&str]) -> Path {
    Path {
        leading_colon: Some(<Token![::]>::default()),
        segments: Punctuated::from_iter(segments.iter().map(|segment| path_segment(segment))),
    }
}

/// Return if given enum has any empty or unit variants. If not an enum, will always return `false`.
pub fn unit_found(data: &Item) -> bool {
    if let Item::Enum { variants, .. } = data {
        variants.iter().any(|variant| variant.fields.is_empty())
    } else {
        false
    }
}
