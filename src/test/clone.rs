use quote::quote;
use syn::Result;

use super::test_derive;

#[test]
fn struct_() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Clone)]
			struct Test<T> { field: std::marker::PhatomData<T> }
		},
		quote! {
			impl<T> ::core::clone::Clone for Test<T> {
				#[inline]
				fn clone(&self) -> Self {
					match self {
						Test { field: ref __field } => Test { field: ::core::clone::Clone::clone(__field) },
					}
				}
			}
		},
	)
}

#[test]
fn tuple() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Clone)]
			struct Test<T>(std::marker::PhatomData<T>);
		},
		quote! {
			impl<T> ::core::clone::Clone for Test<T> {
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
fn enum_() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Clone)]
			enum Test<T> {
				A { field: std::marker::PhatomData<T>},
				B { },
				C(std::marker::PhatomData<T>),
				D(),
				E,
			}
		},
		quote! {
			impl<T> ::core::clone::Clone for Test<T> {
				#[inline]
				fn clone(&self) -> Self {
					match self {
						Test::A { field: ref __field } => Test::A { field: ::core::clone::Clone::clone(__field) },
						Test::B { } => Test::B { },
						Test::C(ref __0) => Test::C(::core::clone::Clone::clone(__0)),
						Test::D() => Test::D(),
						Test::E => Test::E,
					}
				}
			}
		},
	)
}

#[test]
fn union_() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Clone)]
			union Test<T> {
				a: std::marker::PhantomData<T>,
				b: u8,
			}
		},
		quote! {
			impl<T> ::core::clone::Clone for Test<T> {
				#[inline]
				fn clone(&self) -> Self {
					struct __AssertCopy<__T: ::core::marker::Copy + ?::core::marker::Sized>(::core::marker::PhantomData<__T>);
					let _: __AssertCopy<Self>;
				}
			}
		},
	)
}
