//! Utility functions.

use proc_macro2::Span;
use syn::{punctuated::Punctuated, Ident, Path, PathArguments, PathSegment, Token};

/// Convenience type to return two possible values.
pub enum Either<L, R> {
	/// `L` return value.
	Left(L),
	/// `R` return value.
	Right(R),
}

/// Create [`PathSegment`] from [`str`].
pub fn path_segment(ident: &str) -> PathSegment {
	PathSegment {
		ident: Ident::new(ident, Span::call_site()),
		arguments: PathArguments::None,
	}
}

/// Create [`Path`] from `[&str]`s.
pub fn path_from_strs(segments: &[&str]) -> Path {
	Path {
		leading_colon: Some(<Token![::]>::default()),
		segments: Punctuated::from_iter(segments.iter().map(|segment| path_segment(segment))),
	}
}

/// Create [`Path`] from `[&Ident]`s.
pub fn path_from_idents(segments: &[&Ident]) -> Path {
	Path {
		leading_colon: None,
		segments: Punctuated::from_iter(segments.iter().map(|ident| PathSegment {
			ident: (*ident).clone(),
			arguments: PathArguments::None,
		})),
	}
}

/// Create [`Path`] from a root [`Path`] and `[&str]`s.
pub fn path_from_root_and_strs(root: Path, segments: &[&str]) -> Path {
	Path {
		leading_colon: root.leading_colon,
		segments: root
			.segments
			.into_iter()
			.chain(segments.iter().map(|segment| path_segment(segment)))
			.collect(),
	}
}
