//! `impl` generation implementation.

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{punctuated::Punctuated, PredicateType, Result, Token, WhereClause, WherePredicate};

use crate::{Data, DataType, DeriveTrait, Generic, Input, Item, TraitImpl};

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
    pub fn generate_impl(&self) -> Result<TokenStream> {
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
        let body = self.generate_body()?;

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

        Ok(output)
    }

    /// Generate `impl` item body.
    fn generate_body(&self) -> Result<TokenStream> {
        match &self.input.item {
            Item::Struct(data) => {
                let body = self.build_for_struct(data);

                Ok(self.trait_.build_signature(self, &body))
            }
            Item::Tuple(data) => {
                let body = self.build_for_tuple(data);

                Ok(self.trait_.build_signature(self, &body))
            }
            Item::Enum { variants, .. } => {
                let body: TokenStream = variants
                    .iter()
                    .map(|data| {
                        Ok(match data.type_ {
                            DataType::Struct => self.build_for_struct(data),
                            DataType::Tuple => self.build_for_tuple(data),
                            DataType::Unit => self.build_for_unit(data),
                        })
                    })
                    .collect::<Result<_>>()?;

                Ok(self.trait_.build_signature(self, &body))
            }
            Item::Union(_) => self.build_for_union(self.input.span),
        }
    }

    /// Build `match` arms for [`PartialOrd`] and [`Ord`].
    fn build_ord(&self, data: &Data) -> TokenStream {
        use DeriveTrait::*;

        let path = self.trait_.path();
        let mut equal = quote! { ::core::cmp::Ordering::Equal };

        // Add `Option` to `Ordering` if we are implementing `PartialOrd`.
        let method = match self.trait_ {
            PartialOrd => {
                equal = quote! { ::core::option::Option::Some(#equal) };
                quote! { partial_cmp }
            }
            Ord => quote! { cmp },
            _ => unreachable!("unsupported trait in `build_ord`"),
        };

        // The match arm starts with `Ordering::Equal`. This will become the
        // whole `match` arm if no fields are present.
        let mut body = quote! { #equal };

        // Builds `match` arms backwards, using the `match` arm of the field coming afterwards.
        // `rev` has to be called twice separately because it can't be called on `zip`
        for (field_temp, field_other) in data
            .iter_self_ident(self.trait_)
            .rev()
            .zip(data.iter_other_ident(self.trait_).rev())
        {
            body = quote! {
                match #path::#method(#field_temp, #field_other) {
                    #equal => #body,
                    __cmp => __cmp,
                }
            };
        }

        body
    }

    /// Build method body if type is a structure.
    fn build_for_struct(&self, data: &Data) -> TokenStream {
        use DeriveTrait::*;

        let trait_path = self.trait_.path();
        let item_path = &data.path;
        let self_pattern = &data.self_pattern;
        let self_ident = data.iter_self_ident(self.trait_);
        let other_pattern = &data.other_pattern;
        let other_ident = data.iter_other_ident(self.trait_);
        let fields = data.iter_field_ident(self.trait_);

        match self.trait_ {
            Clone => quote! {
                #self_pattern => #item_path { #(#fields: #trait_path::clone(#self_ident)),* },
            },
            Copy => quote! {},
            Debug => {
                let debug_name = data.ident.to_string();
                let debug_fields = fields.map(|field| field.to_string());

                quote! {
                    #self_pattern => {
                        let mut __builder = ::core::fmt::Formatter::debug_struct(__f, #debug_name);
                        #(::core::fmt::DebugStruct::field(&mut __builder, #debug_fields, #self_ident);)*
                        ::core::fmt::DebugStruct::finish(&mut __builder)
                    }
                }
            }
            Default => {
                // Skip `Default` implementation if variant isn't marked with a `default` attribute.
                if data.default.0 {
                    quote! { #item_path { #(#fields: #trait_path::default()),* } }
                } else {
                    quote! {}
                }
            }
            Eq => quote! {},
            Hash => {
                // Add hashing the variant if this is an enum.
                let discriminant = if let Item::Enum { .. } = self.input.item {
                    Some(quote! { #trait_path::hash(&::core::mem::discriminant(self), __state); })
                } else {
                    None
                };

                quote! {
                    #self_pattern => {
                        #discriminant
                        #(#trait_path::hash(#self_ident, __state);)*
                    }
                }
            }
            Ord | PartialOrd => {
                if data.fields.is_empty() {
                    quote! {}
                } else {
                    let body = self.build_ord(data);

                    quote! {
                        (#self_pattern, #other_pattern) => #body,
                    }
                }
            }
            PartialEq => {
                if data.fields.is_empty() {
                    quote! {}
                } else {
                    quote! {
                        (#self_pattern, #other_pattern) =>
                            true #(&& #trait_path::eq(#self_ident, #other_ident))*,
                    }
                }
            }
            #[cfg(feature = "zeroize")]
            Zeroize { .. } => {
                let self_pattern = data.self_pattern_mut();

                let body =
                    data.iter_fields(self.trait_)
                        .zip(self_ident)
                        .map(|(field, self_ident)| {
                            if field.attr.zeroize_fqs.0 {
                                quote! { #trait_path::zeroize(#self_ident); }
                            } else {
                                quote! { #self_ident.zeroize(); }
                            }
                        });

                quote! {
                    #self_pattern => {
                        #(#body)*
                    }
                }
            }
        }
    }

    /// Build method body if type is a tuple.
    fn build_for_tuple(&self, data: &Data) -> TokenStream {
        use DeriveTrait::*;

        let trait_path = self.trait_.path();
        let item_path = &data.path;
        let self_pattern = &data.self_pattern;
        let self_ident = data.iter_self_ident(self.trait_);
        let other_pattern = &data.other_pattern;
        let other_ident = data.iter_other_ident(self.trait_);

        match self.trait_ {
            Clone => quote! {
                #self_pattern => #item_path(#(#trait_path::clone(#self_ident)),*),
            },
            Copy => quote! {},
            Debug => {
                let debug_name = data.ident.to_string();

                quote! {
                    #self_pattern => {
                        let mut __builder = ::core::fmt::Formatter::debug_tuple(__f, #debug_name);
                        #(::core::fmt::DebugTuple::field(&mut __builder, #self_ident);)*
                        ::core::fmt::DebugTuple::finish(&mut __builder)
                    }
                }
            }
            Default => {
                // Skip `Default` implementation if variant isn't marked with a `default` attribute.
                if data.default.0 {
                    let fields = self_ident.map(|_| quote! { #trait_path::default() });

                    quote! { #item_path(#(#fields),*) }
                } else {
                    quote! {}
                }
            }
            Eq => quote! {},
            Hash => {
                // Add hashing the variant if this is an enum.
                let discriminant = if let Item::Enum { .. } = self.input.item {
                    Some(quote! { #trait_path::hash(&::core::mem::discriminant(self), __state); })
                } else {
                    None
                };

                quote! {
                    #self_pattern => {
                        #discriminant
                        #(#trait_path::hash(#self_ident, __state);)*
                    }
                }
            }
            Ord | PartialOrd => {
                if data.fields.is_empty() {
                    quote! {}
                } else {
                    let body = self.build_ord(data);

                    quote! {
                        (#self_pattern, #other_pattern) => #body,
                    }
                }
            }
            PartialEq => {
                if data.fields.is_empty() {
                    quote! {}
                } else {
                    quote! {
                        (#self_pattern, #other_pattern) =>
                            true #(&& #trait_path::eq(#self_ident, #other_ident))*,
                    }
                }
            }
            #[cfg(feature = "zeroize")]
            Zeroize { .. } => {
                let self_pattern = data.self_pattern_mut();

                let body =
                    data.iter_fields(self.trait_)
                        .zip(self_ident)
                        .map(|(field, self_ident)| {
                            if field.attr.zeroize_fqs.0 {
                                quote! { #trait_path::zeroize(#self_ident); }
                            } else {
                                quote! { #self_ident.zeroize(); }
                            }
                        });

                quote! {
                    #self_pattern => {
                        #(#body)*
                    }
                }
            }
        }
    }

    /// Build method body if type is a unit. See description for `pattern` in
    /// [`Self::build_for_struct`].
    fn build_for_unit(&self, data: &Data) -> TokenStream {
        use DeriveTrait::*;

        let trait_path = self.trait_.path();
        let self_pattern = &data.self_pattern;

        match self.trait_ {
            Clone => quote! { #self_pattern => #self_pattern, },
            Copy => quote! {},
            Debug => {
                let debug_name = data.ident.to_string();

                quote! { #self_pattern => ::core::fmt::Formatter::write_str(__f, #debug_name), }
            }
            Default => {
                if data.default.0 {
                    quote! { #self_pattern }
                } else {
                    quote! {}
                }
            }
            Eq => quote! {},
            Hash => {
                // Add hashing the variant if this is an enum.
                let discriminant = if let Item::Enum { .. } = self.input.item {
                    Some(quote! { #trait_path::hash(&::core::mem::discriminant(self), __state); })
                } else {
                    None
                };

                quote! {
                    #self_pattern => {
                        #discriminant
                    }
                }
            }
            Ord => quote! {},
            PartialEq => quote! {},
            PartialOrd => quote! {},
            #[cfg(feature = "zeroize")]
            Zeroize { .. } => quote! {},
        }
    }

    /// Build method body if type is a union. See description for `pattern` in
    /// [`Self::build_for_struct`].
    fn build_for_union(&self, span: Span) -> Result<TokenStream> {
        use DeriveTrait::*;

        match self.trait_ {
            Clone => Ok(quote! {
                #[inline]
                fn clone(&self) -> Self {
                    struct __AssertCopy<__T: ::core::marker::Copy + ?::core::marker::Sized>(::core::marker::PhantomData<__T>);
                    let _: __AssertCopy<Self>;
                    *self
                }
            }),
            Copy => Ok(quote! {}),
            // TODO: convert to proper error message
            _ => Err(syn::Error::new(
                span,
                "traits other then `Clone` and `Copy` aren't supported by unions",
            )),
        }
    }
}
