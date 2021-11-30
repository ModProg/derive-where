//! `impl` generation implementation.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, PredicateType, Token, WhereClause, WherePredicate};

use crate::{DeriveTrait, Generic, Input, Item, TraitImpl};

/// Holds all parameters necessary to generate an implementation for a specific trait.
pub struct Impl<'a> {
    /// [`Trait`](crate::Trait) with options to generate implementation for.
    pub trait_: &'a DeriveTrait,
    /// [`Generic`]s to add.
    pub generics: &'a [Generic],
    /// Item [`Input`].
    pub input: &'a Input<'a>,
}

impl<'a> Impl<'a> {
    /// Create [`Impl`] from parameters.
    pub fn new(trait_: &'a DeriveTrait, generics: &'a [Generic], input: &'a Input<'a>) -> Self {
        Self {
            trait_,
            generics,
            input,
        }
    }

    /// Generate implementation for this trait.
    pub fn generate_impl(&self) -> TokenStream {
        let (impl_generics, type_generics, where_clause) = self.input.generics.split_for_impl();
        let mut where_clause = where_clause.cloned();

        // Only create a where clause if required
        if !self.generics.is_empty() {
            // We use the existing where clause or create a new one if required.
            let where_clause = where_clause.get_or_insert(WhereClause {
                where_token: <Token![where]>::default(),
                predicates: Punctuated::default(),
            });

            // Insert bounds into the `where` clause.
            for generic in self.generics {
                where_clause
                    .predicates
                    .push(WherePredicate::Type(match generic {
                        Generic::CoustomBound(type_bound) => type_bound.clone(),
                        Generic::NoBound(path) => PredicateType {
                            lifetimes: None,
                            bounded_ty: path.clone(),
                            colon_token: <Token![:]>::default(),
                            bounds: self.trait_.where_bounds(&self.input.item),
                        },
                    }));
            }
        }

        let item = self.input.item.ident();
        let path = self.trait_.path();
        let body = self.generate_body();

        let mut output = quote! {
            impl #impl_generics #path for #item #type_generics
            #where_clause
            {
                #body
            }
        };

        if let Some((path, body)) = self.trait_.additional_impl(self.trait_) {
            output.extend(quote! {
                impl #impl_generics #path for #item #type_generics
                #where_clause
                {
                    #body
                }
            })
        }

        output
    }

    /// Generate `impl` item body.
    fn generate_body(&self) -> TokenStream {
        match &self.input.item {
            Item::Item(data) => {
                let body = self.trait_.build_body(self.trait_, data);
                self.trait_.build_signature(self, &body)
            }
            Item::Enum { variants, .. } => {
                let body: TokenStream = variants
                    .iter()
                    .map(|data| self.trait_.build_body(self.trait_, data))
                    .collect();

                self.trait_.build_signature(self, &body)
            }
        }
    }
}
