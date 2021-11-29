//! Types holding data of items.

use syn::{
    token::{Brace, Paren},
    FieldPat, FieldsNamed, FieldsUnnamed, Ident, Pat, PatIdent, PatPath, PatStruct, PatTuple,
    PatTupleStruct, Path, Result, Token,
};

use crate::{Default, Field, Member, Skip, Trait};

/// Holds all relevant data of a struct, union or variant.
#[cfg_attr(test, derive(Debug))]
pub struct Data<'a> {
    /// [`Skip`] attribute of this struct, union or variant.
    pub skip_inner: Skip,
    /// [`Default`](crate::Default) attribute of this variant, always `true` for structs or unions.
    pub default: Default,
    /// [`struct@Ident`] of this struct, union or variant.
    pub ident: &'a Ident,
    /// [`Type`](DataType) of this struct, union or variant.
    pub type_: DataType,
    /// [`Path`] of this struct, union or variant.
    pub path: Path,
    /// [Pattern](Pat) to use in a match arm to destructure `self`.
    pub self_pattern: Pat,
    /// [Pattern](Pat) to use in a match arm to destructure `other`.
    pub other_pattern: Pat,
    /// [`Field`]s of this struct, union or variant.
    pub fields: Vec<Field<'a>>,
}

/// Type of [`Data`].
#[cfg_attr(test, derive(Debug))]
pub enum DataType {
    /// Struct, union or struct variant.
    Struct,
    /// Tuple struct or tuple variant.
    Tuple,
    /// Unit variant.
    Unit,
}

impl<'a> Data<'a> {
    /// Create [`Data`]s from [`FieldsNamed`].
    pub fn from_named(
        skip_inner: Skip,
        default: Default,
        ident: &'a Ident,
        path: Path,
        fields: &'a FieldsNamed,
    ) -> Result<Self> {
        /// Destructuring pattern in a match arm for this item or variant.
        fn pattern(
            path: Path,
            fields: &[Field],
            field_ident: impl for<'a> Fn(&'a Field) -> &'a Ident,
        ) -> Pat {
            Pat::Struct(PatStruct {
                attrs: Vec::new(),
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
                dot2_token: None,
            })
        }

        let fields = Field::from_named(fields)?;

        let self_pattern = pattern(path.clone(), &fields, |field| &field.self_ident);
        let other_pattern = pattern(path.clone(), &fields, |field| &field.other_ident);

        Ok(Self {
            skip_inner,
            default,
            ident,
            type_: DataType::Struct,
            path,
            self_pattern,
            other_pattern,
            fields,
        })
    }

    /// Create [`Data`]s from [`FieldsUnnamed`].
    pub fn from_unnamed(
        skip_inner: Skip,
        default: Default,
        ident: &'a Ident,
        path: Path,
        fields: &'a FieldsUnnamed,
    ) -> Result<Self> {
        /// Destructuring pattern in a match arm for this item or variant.
        fn pattern(
            path: Path,
            fields: &[Field],
            field_ident: impl for<'a> Fn(&'a Field) -> &'a Ident,
        ) -> Pat {
            Pat::TupleStruct(PatTupleStruct {
                attrs: Vec::new(),
                path,
                pat: PatTuple {
                    attrs: Vec::new(),
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
                },
            })
        }

        let fields = Field::from_unnamed(fields)?;

        let self_pattern = pattern(path.clone(), &fields, |field| &field.self_ident);
        let other_pattern = pattern(path.clone(), &fields, |field| &field.other_ident);

        Ok(Self {
            skip_inner,
            default,
            ident,
            type_: DataType::Tuple,
            path,
            self_pattern,
            other_pattern,
            fields,
        })
    }

    /// Create [`Data`]s from unit.
    pub fn from_unit(
        skip_inner: Skip,
        default: Default,
        ident: &'a Ident,
        path: Path,
    ) -> Result<Self> {
        let pattern = Pat::Path(PatPath {
            attrs: Vec::new(),
            qself: None,
            path: path.clone(),
        });

        Ok(Self {
            skip_inner,
            default,
            ident,
            type_: DataType::Unit,
            path,
            self_pattern: pattern.clone(),
            other_pattern: pattern,
            fields: Vec::new(),
        })
    }

    /// Returns a [Pattern](Pat) to use in a match arm to destructure `self` as mutable.
    #[cfg(feature = "zeroize")]
    pub fn self_pattern_mut(&self) -> Pat {
        match self.type_ {
            DataType::Struct | DataType::Tuple => {
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
                        for field in &mut pattern.pat.elems {
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
            DataType::Unit => self.self_pattern.clone(),
        }
    }

    /// Returns an [`Iterator`] over [`Field`]s.
    #[cfg(feature = "zeroize")]
    pub fn iter_fields(&'a self, trait_: &'a Trait) -> impl 'a + Iterator<Item = &'a Field> {
        self.fields.iter().filter(move |field| !field.skip(trait_))
    }

    /// Returns an [`Iterator`] over [`Member`]s.
    pub fn iter_field_ident(&'a self, trait_: &'a Trait) -> impl 'a + Iterator<Item = &'a Member> {
        self.fields.iter().filter_map(move |field| {
            if field.skip(trait_) {
                None
            } else {
                Some(&field.member)
            }
        })
    }

    /// Returns an [`Iterator`] over [`struct@Ident`]s used as temporary variables for destructuring `self`.
    pub fn iter_self_ident(
        &'a self,
        trait_: &'a Trait,
    ) -> impl Iterator<Item = &'a Ident> + DoubleEndedIterator {
        self.fields.iter().filter_map(move |field| {
            if field.skip(trait_) {
                None
            } else {
                Some(&field.self_ident)
            }
        })
    }

    /// Returns an [`Iterator`] over [`struct@Ident`]s used as temporary variables for destructuring `other`.
    pub fn iter_other_ident(
        &'a self,
        trait_: &'a Trait,
    ) -> impl Iterator<Item = &'a Ident> + DoubleEndedIterator {
        self.fields.iter().filter_map(move |field| {
            if field.skip(trait_) {
                None
            } else {
                Some(&field.other_ident)
            }
        })
    }
}
