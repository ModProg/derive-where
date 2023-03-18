//! Storage for items or variants with data.

use syn::{
	token::{Brace, Paren},
	FieldPat, FieldsNamed, FieldsUnnamed, Ident, Pat, PatIdent, PatStruct, PatTupleStruct, Path,
	Result, Token,
};
#[cfg(all(not(feature = "nightly"), feature = "safe"))]
use {std::iter, syn::punctuated::Punctuated, syn::PatRest};

use crate::{DeriveWhere, Field, Skip, Trait};

/// Struct, union, struct variant or tuple variant fields.
#[cfg_attr(test, derive(Debug))]
pub struct Fields<'a> {
	/// [Pattern](Pat) to use in a match arm to destructure `self`.
	pub self_pattern: Pat,
	/// [Pattern](Pat) to use in a match arm to destructure `other`.
	pub other_pattern: Pat,
	/// [Pattern](Pat) to use in a match arm to destructure `other` but skipping
	/// all fields.
	#[cfg(all(not(feature = "nightly"), feature = "safe"))]
	pub other_pattern_skip: Pat,
	/// [`Field`]s of this struct, union or variant.
	pub fields: Vec<Field<'a>>,
}

impl<'a> Fields<'a> {
	/// Create [`Fields`]s from [`FieldsNamed`].
	pub fn from_named(
		derive_wheres: &[DeriveWhere],
		skip_inner: &Skip,
		path: Path,
		fields: &'a FieldsNamed,
	) -> Result<Self> {
		let fields = Field::from_named(derive_wheres, skip_inner, fields)?;

		let self_pattern = Self::struct_pattern(path.clone(), &fields, |field| &field.self_ident);
		#[cfg(all(not(feature = "nightly"), feature = "safe"))]
		let other_pattern_skip = Pat::Struct(PatStruct {
			attrs: Vec::new(),
			qself: None,
			path: path.clone(),
			brace_token: Brace::default(),
			fields: Punctuated::new(),
			rest: Some(PatRest {
				attrs: Vec::new(),
				dot2_token: <Token![..]>::default(),
			}),
		});
		let other_pattern = Self::struct_pattern(path, &fields, |field| &field.other_ident);

		Ok(Self {
			self_pattern,
			other_pattern,
			#[cfg(all(not(feature = "nightly"), feature = "safe"))]
			other_pattern_skip,
			fields,
		})
	}

	/// Create [`Fields`]s from [`FieldsUnnamed`].
	pub fn from_unnamed(
		derive_wheres: &[DeriveWhere],
		skip_inner: &Skip,
		path: Path,
		fields: &'a FieldsUnnamed,
	) -> Result<Self> {
		let fields = Field::from_unnamed(derive_wheres, skip_inner, fields)?;

		let self_pattern = Self::tuple_pattern(path.clone(), &fields, |field| &field.self_ident);
		#[cfg(all(not(feature = "nightly"), feature = "safe"))]
		let other_pattern_skip = Pat::TupleStruct(PatTupleStruct {
			attrs: Vec::new(),
			qself: None,
			path: path.clone(),
			paren_token: Paren::default(),
			elems: iter::once(Pat::Rest(PatRest {
				attrs: Vec::new(),
				dot2_token: <Token![..]>::default(),
			}))
			.collect(),
		});
		let other_pattern = Self::tuple_pattern(path, &fields, |field| &field.other_ident);

		Ok(Self {
			self_pattern,
			other_pattern,
			#[cfg(all(not(feature = "nightly"), feature = "safe"))]
			other_pattern_skip,
			fields,
		})
	}

	/// Destructuring pattern in a match arm for this item or variant.
	fn struct_pattern(
		path: Path,
		fields: &[Field],
		field_ident: impl for<'b> Fn(&'b Field) -> &'b Ident,
	) -> Pat {
		Pat::Struct(PatStruct {
			attrs: Vec::new(),
			qself: None,
			path,
			brace_token: Brace::default(),
			fields: fields
				.iter()
				.map(|field| FieldPat {
					attrs: Vec::new(),
					member: field.to_member(),
					colon_token: Some(<Token![:]>::default()),
					pat: Box::new(Pat::Ident(PatIdent {
						attrs: Vec::new(),
						by_ref: Some(<Token![ref]>::default()),
						mutability: None,
						ident: field_ident(field).clone(),
						subpat: None,
					})),
				})
				.collect(),
			rest: None,
		})
	}

	/// Destructuring pattern in a match arm for this item or variant.
	fn tuple_pattern(
		path: Path,
		fields: &[Field],
		field_ident: impl for<'b> Fn(&'b Field) -> &'b Ident,
	) -> Pat {
		Pat::TupleStruct(PatTupleStruct {
			attrs: Vec::new(),
			qself: None,
			path,
			paren_token: Paren::default(),
			elems: fields
				.iter()
				.map(|field| {
					Pat::Ident(PatIdent {
						attrs: Vec::new(),
						by_ref: Some(<Token![ref]>::default()),
						mutability: None,
						ident: field_ident(field).clone(),
						subpat: None,
					})
				})
				.collect(),
		})
	}

	/// Returns a [Pattern](Pat) to use in a match arm to destructure `self` as
	/// mutable.
	#[cfg(feature = "zeroize")]
	pub fn self_pattern_mut(&self) -> Pat {
		let mut pattern = self.self_pattern.clone();

		match &mut pattern {
			Pat::Struct(pattern) => {
				for field in &mut pattern.fields {
					if let Pat::Ident(pattern) = &mut *field.pat {
						pattern.mutability = Some(<Token![mut]>::default());
					} else {
						unreachable!("unexpected pattern")
					}
				}
			}
			Pat::TupleStruct(pattern) => {
				for field in &mut pattern.elems {
					if let Pat::Ident(pattern) = &mut *field {
						pattern.mutability = Some(<Token![mut]>::default());
					} else {
						unreachable!("unexpected pattern")
					}
				}
			}
			_ => unreachable!("unexpected pattern"),
		}

		pattern
	}

	/// Returns `true` if any field is skipped with that [`Trait`].
	pub fn any_skip_trait(&self, trait_: Trait) -> bool {
		self.fields.iter().any(|field| field.skip(trait_))
	}

	/// Returns `true` if all fields are skipped with that [`Trait`].
	pub fn skip(&self, trait_: Trait) -> bool {
		self.fields.iter().all(|field| field.skip(trait_))
	}
}
