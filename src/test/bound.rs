use quote::quote;
use syn::Result;

use super::test_derive;

#[test]
fn no_bound() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Clone)]
			struct Test<T>(u8, core::marker::PhantomData<T>);
		},
		quote! {
			impl<T> ::core::clone::Clone for Test<T>
			{
				#[inline]
				fn clone(&self) -> Self {
					match self {
						Test(ref __0, ref __1) => Test(::core::clone::Clone::clone(__0), ::core::clone::Clone::clone(__1)),
					}
				}
			}
		},
	)
}

#[test]
fn no_bound_multiple() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Clone, Copy)]
			struct Test<T>(u8, core::marker::PhantomData<T>);
		},
		quote! {
			impl<T> ::core::clone::Clone for Test<T>
			{
				#[inline]
				fn clone(&self) -> Self {
					match self {
						Test(ref __0, ref __1) => Test(::core::clone::Clone::clone(__0), ::core::clone::Clone::clone(__1)),
					}
				}
			}

			impl<T> ::core::marker::Copy for Test<T> { }
		},
	)
}

#[test]
fn custom_bound() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Clone; T: Copy)]
			struct Test<T>(T);
		},
		quote! {
			impl<T> ::core::clone::Clone for Test<T>
			where T: Copy
			{
				#[inline]
				fn clone(&self) -> Self {
					match self {
						Test(ref __0) => Test(::core::clone::Clone::clone(__0)),
					}
				}
			}
		},
	)
}

#[test]
fn where_() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Clone; T)]
			struct Test<T>(T) where T: core::fmt::Debug;
		},
		quote! {
			impl<T> ::core::clone::Clone for Test<T>
			where
				T: core::fmt::Debug,
				T: ::core::clone::Clone
			{
				#[inline]
				fn clone(&self) -> Self {
					match self {
						Test(ref __0) => Test(::core::clone::Clone::clone(__0)),
					}
				}
			}
		},
	)
}

#[test]
fn associated_type() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Clone; <T as core::ops::Deref>::Target)]
			struct Test<T>(<T as core::ops::Deref>::Target);
		},
		quote! {
			impl<T> ::core::clone::Clone for Test<T>
			where <T as core::ops::Deref>::Target: ::core::clone::Clone
			{
				#[inline]
				fn clone(&self) -> Self {
					match self {
						Test(ref __0) => Test(::core::clone::Clone::clone(__0)),
					}
				}
			}
		},
	)
}

#[test]
fn associated_type_custom_bound() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Clone; <T as core::ops::Deref>::Target: Copy)]
			struct Test<T>(<T as core::ops::Deref>::Target);
		},
		quote! {
			impl<T> ::core::clone::Clone for Test<T>
			where <T as core::ops::Deref>::Target: Copy
			{
				#[inline]
				fn clone(&self) -> Self {
					match self {
						Test(ref __0) => Test(::core::clone::Clone::clone(__0)),
					}
				}
			}
		},
	)
}
