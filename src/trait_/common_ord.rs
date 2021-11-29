//! Common implementation help for [`PartialOrd`] and [`Ord`].

use proc_macro2::TokenStream;
use quote::quote;

use crate::{util, Impl, Item};

/// Build signature for [`PartialOrd`] and [`Ord`].
pub fn build_ord_signature(impl_: &Impl, body: &TokenStream) -> TokenStream {
    use crate::DeriveTrait::*;

    let mut equal = quote! { ::core::cmp::Ordering::Equal };

    // Add `Option` to `Ordering` if we are implementing `PartialOrd`.
    if let PartialOrd = impl_.trait_ {
        equal = quote! { ::core::option::Option::Some(#equal) };
    }

    match &impl_.input.item {
        // Only check for discriminators if there is more than one variant.
        Item::Enum { variants, .. } if variants.len() > 1 => {
            let rest = if util::unit_found(&impl_.input.item) {
                quote! { #equal }
            } else {
                #[cfg(not(feature = "safe"))]
                // This follows the standard implementation.
                quote! { unsafe { ::core::hint::unreachable_unchecked() } }
                #[cfg(feature = "safe")]
                quote! { ::core::unreachable!("comparing variants yielded unexpected results") }
            };

            #[cfg(any(feature = "nightly", not(feature = "safe")))]
            {
                let path = impl_.trait_.path();
                let method = match impl_.trait_ {
                    PartialOrd => quote! { partial_cmp },
                    Ord => quote! { cmp },
                    _ => unreachable!("unsupported trait in `prepare_ord`"),
                };

                #[cfg(feature = "nightly")]
                quote! {
                    let __self_disc = ::core::intrinsics::discriminant_value(&self);
                    let __other_disc = ::core::intrinsics::discriminant_value(&__other);

                    if __self_disc == __other_disc {
                        match (self, __other) {
                            #body
                            _ => #rest,
                        }
                    } else {
                        #path::#method(&__self_disc, &__other_disc)
                    }
                }
                #[cfg(not(feature = "nightly"))]
                quote! {
                    let __self_disc = ::core::mem::discriminant(self);
                    let __other_disc = ::core::mem::discriminant(__other);

                    if __self_disc == __other_disc {
                        match (self, __other) {
                            #body
                            _ => #rest,
                        }
                    } else {
                        #path::#method(
                            &unsafe { ::core::mem::transmute::<_, isize>(__self_disc) },
                            &unsafe { ::core::mem::transmute::<_, isize>(__other_disc) },
                        )
                    }
                }
            }
            #[cfg(all(not(feature = "nightly"), feature = "safe"))]
            {
                let mut less = quote! { ::core::cmp::Ordering::Less };
                let mut greater = quote! { ::core::cmp::Ordering::Greater };

                // Add `Option` to `Ordering` if we are implementing `PartialOrd`.
                if let PartialOrd = impl_.trait_ {
                    less = quote! { ::core::option::Option::Some(#less) };
                    greater = quote! { ::core::option::Option::Some(#greater) };
                }

                let mut different = Vec::with_capacity(variants.len());

                // Build separate `match` arms to compare different variants to each
                // other. The index for these variants is used to determine which
                // `Ordering` to return.
                for (index, variant) in variants.iter().enumerate() {
                    let mut arms = Vec::with_capacity(variants.len() - 1);

                    for (index_other, variant_other) in variants.iter().enumerate() {
                        // Make sure we aren't comparing the same variant with itself.
                        if index != index_other {
                            use core::cmp::Ordering::*;

                            let ordering = match index.cmp(&index_other) {
                                Less => &less,
                                Equal => &equal,
                                Greater => &greater,
                            };

                            let pattern = &variant_other.other_pattern;

                            arms.push(quote! {
                                #pattern => #ordering,
                            });
                        }
                    }

                    let pattern = &variant.self_pattern;

                    different.push(quote! {
                                #pattern => match __other {
                                    #(#arms)*
                                    _ => ::core::unreachable!("comparing variants yielded unexpected results"),
                                },
                            });
                }

                quote! {
                    let __self_disc = ::core::mem::discriminant(self);
                    let __other_disc = ::core::mem::discriminant(__other);

                    if __self_disc == __other_disc {
                        match (self, __other) {
                            #body
                            _ => #rest,
                        }
                    } else {
                        match self {
                            #(#different)*
                        }
                    }
                }
            }
        }
        // If only one variant was found and it's a unit variant, return `Eq`.
        Item::Enum { variants, .. }
            if variants.len() == 1 && util::unit_found(&impl_.input.item) =>
        {
            quote! {
                #equal
            }
        }
        _ => {
            quote! {
                match (self, __other) {
                    #body
                }
            }
        }
    }
}
