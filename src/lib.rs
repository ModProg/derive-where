#![deny(unsafe_code)]
#![warn(clippy::cargo, clippy::missing_docs_in_private_items)]
#![cfg_attr(doc, warn(rustdoc::all), allow(rustdoc::missing_doc_code_examples))]

//! TODO

use core::cmp::Ordering;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    Data, DeriveInput, Error, Fields, FieldsNamed, FieldsUnnamed, Result, Token, TraitBound, Type,
};

/// Holds parsed [bounds](TraitBound) and [traits][Traits].
struct DeriveWhere {
    /// Parsed [bounds](TraitBound).
    bounds: Vec<TraitBound>,
    /// Parsed [traits][Traits].
    traits: Vec<Traits>,
}

impl Parse for DeriveWhere {
    fn parse(input: ParseStream) -> Result<Self> {
        let bounds = Punctuated::<TraitBound, Token![,]>::parse_separated_nonempty(input)?
            .into_iter()
            .collect();

        <Token![;]>::parse(input)?;

        let traits = Punctuated::<Traits, Token![,]>::parse_terminated(input)?
            .into_iter()
            .collect();

        Ok(Self { bounds, traits })
    }
}

/// Trait to implement.
#[derive(Copy, Clone, Debug)]
enum Traits {
    /// [`Clone`].
    Clone,
    /// [`Copy`].
    Copy,
    /// [`Debug`](core::fmt::Debug).
    Debug,
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
}

impl Parse for Traits {
    fn parse(input: ParseStream) -> Result<Self> {
        use Traits::*;

        let ident = Ident::parse(input)?;

        Ok(match ident.to_string().as_str() {
            "Clone" => Clone,
            "Copy" => Copy,
            "Debug" => Debug,
            "Eq" => Eq,
            "Hash" => Hash,
            "Ord" => Ord,
            "PartialEq" => PartialEq,
            "PartialOrd" => PartialOrd,
            ident => {
                return Err(Error::new(
                    ident.span(),
                    format!("{} isn't supported", ident),
                ))
            }
        })
    }
}

impl Traits {
    /// Returns corresponding fully qualified path to the trait.
    fn type_(self) -> Type {
        use Traits::*;

        syn::parse_str(match self {
            Clone => "::core::clone::Clone",
            Copy => "::core::copy::Copy",
            Debug => "::core::fmt::Debug",
            Eq => "::core::cmp::Eq",
            Hash => "::core::hash::Hash",
            Ord => "::core::cmp::Ord",
            PartialEq => "::core::cmp::PartialEq",
            PartialOrd => "::core::cmp::PartialOrd",
        })
        .expect("failed to parse path")
    }

    /// Generate `impl` item body.
    fn generate_body(self, debug_name: &Ident, data: &Data) -> Result<TokenStream> {
        let body = match &data {
            Data::Struct(data) => {
                let pattern = quote! { Self };

                match &data.fields {
                    Fields::Named(fields) => self.build_struct(debug_name, &pattern, None, fields),
                    Fields::Unnamed(fields) => self.generate_tuple(debug_name, &pattern, None, fields),
                    fields @ Fields::Unit => return Err(Error::new(
                        fields.span(),
                        "Using `derive_where` on unit struct is not supported as unit structs don't support generics.")),
                }
            }
            Data::Enum(data) => {
                // Collect all variants to build `PartialOrd` and `Ord`.
                let variants: Vec<_> = data.variants.iter().map(|variant| &variant.ident).collect();

                data.variants
                    .iter()
                    .enumerate()
                    .map(|(index, variant)| {
                        let debug_name = &variant.ident;
                        let pattern = quote! { Self::#debug_name };

                        match &variant.fields {
                            Fields::Named(fields) => self.build_struct(
                                debug_name,
                                &pattern,
                                Some((index, &variants)),
                                fields,
                            ),
                            Fields::Unnamed(fields) => self.generate_tuple(
                                debug_name,
                                &pattern,
                                Some((index, &variants)),
                                fields,
                            ),
                            Fields::Unit => {
                                self.generate_unit(debug_name, &pattern, Some((index, &variants)))
                            }
                        }
                    })
                    .collect()
            }
            Data::Union(fields) => {
                // TODO: this span is probably not ideal.
                return Err(Error::new(fields.fields.span(), "Unions aren't supported."));
            }
        };

        Ok(self.build_signature(body))
    }

    /// Build `match` arms for [`PartialOrd`] and [`Ord`].
    fn prepare_ord(
        self,
        fields_temp: &[Ident],
        fields_other: &[Ident],
        variants: Option<(usize, &[&Ident])>,
        skip: &TokenStream,
    ) -> (TokenStream, TokenStream) {
        use Traits::*;

        let type_ = self.type_();

        let mut less = quote! { ::core::cmp::Ordering::Less };
        let mut equal = quote! { ::core::cmp::Ordering::Equal };
        let mut greater = quote! { ::core::cmp::Ordering::Greater };

        // Add `Option` to `Ordering` if we are implementing `PartialOrd`.
        match self {
            PartialOrd => {
                less = quote! { ::core::option::Option::Some(#less) };
                equal = quote! { ::core::option::Option::Some(#equal) };
                greater = quote! { ::core::option::Option::Some(#greater) };
            }
            Ord => (),
            _ => unreachable!("Unsupported trait in `prepare_ord`."),
        };

        // The match arm starts with `Ordering::Equal`. This will become the
        // whole `match` arm if no fields are present.
        let mut body = quote! { #equal };

        // Builds `match` arms backwards, using the `match` arm of the field coming afterwards.
        for (field_temp, field_other) in fields_temp.iter().zip(fields_other).rev() {
            body = quote! {
                match #type_::partial_cmp(&#field_temp, &#field_other) {
                    #equal => #body,
                    __cmp => __cmp,
                }
            };
        }

        let mut other = quote! {};

        // Build separate `match` arms to compare different variants to each
        // other. The index for these variants is used to determine which
        // `Ordering` to return.
        if let Some((variant, variants)) = variants {
            for (index, variants) in variants.iter().enumerate() {
                // Make sure we aren't comparing the same variant with itself.
                if variant != index {
                    let ordering = match variant.cmp(&index) {
                        Ordering::Less => &less,
                        Ordering::Equal => &equal,
                        Ordering::Greater => &greater,
                    };

                    other.extend(quote! {
                        Self::#variants #skip => #ordering,
                    })
                }
            }
        }

        (body, other)
    }

    /// Build method signature of the corresponding trait.
    fn build_signature(self, body: TokenStream) -> TokenStream {
        use Traits::*;

        let type_ = self.type_();

        match self {
            Clone => quote! {
                fn clone(&self) -> Self {
                    match self {
                        #body
                    }
                }
            },
            Copy => quote! {},
            Debug => quote! {
                fn fmt(&self, __f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        #body
                    }
                }
            },
            Eq => quote! {},
            Hash => quote! {
                fn hash<__H: ::core::hash::Hasher>(&self, __state: &mut __H) {
                    #type_::hash(&::core::mem::discriminant(self), __state);

                    match self {
                        #body
                    }
                }
            },
            Ord => quote! {
                fn cmp(&self, __other: &Self) -> ::core::cmp::Ordering {
                    match (self, __other) {
                        #body
                    }
                }
            },
            PartialEq => quote! {
                fn eq(&self, __other: &Self) -> bool {
                    if ::core::mem::discriminant(self) == ::core::mem::discriminant(__other) {
                        let mut __cmp = true;

                        match (self, __other) {
                            #body
                            _ => ::core::unreachable("Comparing discriminants failed")
                        }
                    } else {
                        false
                    }
                }
            },
            PartialOrd => quote! {
                fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                    match self {
                        #body
                    }
                }
            },
        }
    }

    /// Build method body if type is a structure.
    fn build_struct(
        self,
        debug_name: &Ident,
        pattern: &TokenStream,
        variants: Option<(usize, &[&Ident])>,
        fields: &FieldsNamed,
    ) -> TokenStream {
        use Traits::*;

        let type_ = self.type_();
        let debug_name = debug_name.to_string();

        // Extract `Ident`s from fields.
        let fields: Vec<_> = fields
            .named
            .iter()
            .map(|field| field.ident.as_ref().expect("missing field name"))
            .collect();

        // Build temporary de-structuring variable names from field `Ident`s.
        let fields_temp: Vec<_> = fields
            .iter()
            .map(|field| format_ident!("__{}", field))
            .collect();

        // Build temporary de-structuring variable names for when comparing to the
        // other value, e.g. in `PartialEq`.
        let fields_other: Vec<_> = fields
            .iter()
            .map(|field| format_ident!("__other_{}", field))
            .collect();

        match self {
            Clone => quote! {
                #pattern { #(#fields: #fields_temp),* } => #pattern { #(#fields: #type_::clone(&#fields_temp)),* },
            },
            Copy => quote! {},
            Debug => quote! {
                #pattern { #(#fields: #fields_temp),* } => {
                    let mut __builder = ::core::fmt::Formatter::debug_struct(__f, #debug_name);
                    #(::core::fmt::DebugStruct::field(&mut __builder, #fields, &#fields_temp);)*
                    ::core::fmt::DebugStruct::finish(&mut __builder)
                }
            },
            Eq => quote! {},
            Hash => quote! {
                #pattern { #(#fields: #fields_temp),* } => { #(#type_::hash(&#fields_temp, __state);)* }
            },
            Ord => {
                let (body, other) =
                    self.prepare_ord(&fields_temp, &fields_other, variants, &quote! { { .. } });

                quote! {
                    #pattern { #(#fields: #fields_temp),* } => {
                        match __other {
                            #pattern { #(#fields: #fields_other),* } => #body,
                            #other
                        }
                    }
                }
            }
            PartialEq => quote! {
                (#pattern { #(#fields: #fields_temp),* }, #pattern { #(#fields: #fields_other),* }) => {
                    #(__cmp &= #type_::eq(&#fields_temp, &#fields_other);)*
                }
            },
            PartialOrd => {
                let (body, other) =
                    self.prepare_ord(&fields_temp, &fields_other, variants, &quote! { { .. } });

                quote! {
                    #pattern { #(#fields: #fields_temp),* } => {
                        match __other {
                            #pattern { #(#fields: #fields_other),* } => #body,
                            #other
                        }
                    }
                }
            }
        }
    }

    /// Build method body if type is a tuple.
    fn generate_tuple(
        self,
        debug_name: &Ident,
        pattern: &TokenStream,
        variants: Option<(usize, &[&Ident])>,
        fields: &FieldsUnnamed,
    ) -> TokenStream {
        use Traits::*;

        let type_ = self.type_();
        let debug_name = debug_name.to_string();

        // Build temporary de-structuring variable names from field indexes.
        let fields_temp: Vec<_> = (0..fields.unnamed.len())
            .into_iter()
            .map(|field| format_ident!("__{}", field))
            .collect();

        // Build temporary de-structuring variable names for when comparing to the
        // other value, e.g. in `PartialEq`.
        let fields_other: Vec<_> = (0..fields.unnamed.len())
            .into_iter()
            .map(|field| format_ident!("__other_{}", field))
            .collect();

        match self {
            Clone => quote! {
                #pattern(#(#fields_temp),*) => #pattern (#(#type_::clone(&#fields_temp)),*),
            },
            Copy => quote! {},
            Debug => quote! {
                #pattern(#(#fields_temp),*) => {
                    let mut __builder = ::core::fmt::Formatter::debug_tuple(__f, #debug_name);
                    #(::core::fmt::DebugTuple::field(&mut __builder, &#fields_temp);)*
                    ::core::fmt::DebugTuple::finish(&mut __builder)
                }
            },
            Eq => quote! {},
            Hash => quote! {
                #pattern(#(#fields_temp),*) => { #(#type_::hash(&#fields_temp, __state);)* }
            },
            Ord => {
                let (body, other) =
                    self.prepare_ord(&fields_temp, &fields_other, variants, &quote! { (..) });

                quote! {
                    #pattern (#(#fields_other),*) => {
                        match __other {
                            #pattern (#(#fields_other),*) => #body,
                            #other
                        }
                    }
                }
            }
            PartialEq => quote! {
                (#pattern(#(#fields_temp),*), #pattern(#(#fields_other),*)) => {
                    #(__cmp &= #type_::eq(&#fields_temp, &#fields_other);)*
                }
            },
            PartialOrd => {
                let (body, other) =
                    self.prepare_ord(&fields_temp, &fields_other, variants, &quote! { (..) });

                quote! {
                    #pattern (#(#fields_other),*) => {
                        match __other {
                            #pattern (#(#fields_other),*) => #body,
                            #other
                        }
                    }
                }
            }
        }
    }

    /// Build method body if type is a unit.
    fn generate_unit(
        self,
        debug_name: &Ident,
        pattern: &TokenStream,
        variants: Option<(usize, &[&Ident])>,
    ) -> TokenStream {
        use Traits::*;

        let debug_name = debug_name.to_string();

        match self {
            Clone => quote! { #pattern => #pattern, },
            Copy => quote! {},
            Debug => quote! { #pattern => ::core::fmt::Formatter::write_str(__f, #debug_name), },
            Eq => quote! {},
            Hash => quote! { #pattern => (), },
            Ord => {
                let (body, other) = self.prepare_ord(&[], &[], variants, &quote! {});

                quote! {
                    #pattern => {
                        match __other {
                            #pattern => #body,
                            #other
                        }
                    }
                }
            }
            PartialEq => quote! { (#pattern, #pattern) => true, },
            PartialOrd => {
                let (body, other) = self.prepare_ord(&[], &[], variants, &quote! {});

                quote! {
                    #pattern => {
                        match __other {
                            #pattern => #body,
                            #other
                        }
                    }
                }
            }
        }
    }
}

/// Internal derive function for handling errors.
fn derive_where_internal(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
    // The item needs to be added, as it is consumed by the derive.
    let mut output = quote! { #item };

    // Parse derive macro.
    let derive_where: DeriveWhere = syn::parse2(attr)?;

    // Parse item.
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = syn::parse2(item)?;

    // Every trait needs a separate implementation.
    for trait_ in &derive_where.traits {
        let body = trait_.generate_body(&ident, &data)?;
        let trait_ = trait_.type_();

        // Build necessary generics to construct the implementation item.
        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
        // TODO: don't convert to `TokenStream`, but actually properly insert bounds.
        let mut where_clause = where_clause.map(|where_clause| where_clause.into_token_stream());

        // If there are any bounds, insert them into the `where` clause.
        if !derive_where.bounds.is_empty() {
            // If there is no `where` clause, make one.
            let where_clause = where_clause.get_or_insert(quote! { where });

            for bound in &derive_where.bounds {
                where_clause.extend(quote! {
                    #bound: #trait_,
                })
            }
        }

        // Add implementation item to the output.
        output.extend(quote! {
            impl #impl_generics #trait_ for #ident #type_generics
            #where_clause
            {
                #body
            }
        })
    }

    Ok(output)
}

/// TODO
#[proc_macro_attribute]
pub fn derive_where(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // Redirect to `derive_where_internal`, this only convert the error appropriately.
    match derive_where_internal(attr.into(), item.into()) {
        Ok(output) => output.into(),
        Err(error) => error.to_compile_error().into(),
    }
}
