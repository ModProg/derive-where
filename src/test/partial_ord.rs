use quote::quote;
use syn::Result;

use super::test_derive;

#[test]
fn struct_() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(PartialOrd)]
			struct Test<T> { field: std::marker::PhatomData<T> }
		},
		quote! {
			impl<T> ::core::cmp::PartialOrd for Test<T> {
				#[inline]
				fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
					match (self, __other) {
						(Test { field: ref __field }, Test { field: ref __other_field }) =>
							match ::core::cmp::PartialOrd::partial_cmp(__field, __other_field) {
								::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
								__cmp => __cmp,
							},
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
			#[derive_where(PartialOrd)]
			struct Test<T>(std::marker::PhatomData<T>);
		},
		quote! {
			impl<T> ::core::cmp::PartialOrd for Test<T> {
				#[inline]
				fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
					match (self, __other) {
						(Test(ref __0), Test(ref __other_0)) =>
							match ::core::cmp::PartialOrd::partial_cmp(__0, __other_0) {
								::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
								__cmp => __cmp,
							},
					}
				}
			}
		},
	)
}

#[test]
fn enum_() -> Result<()> {
	#[cfg(feature = "nightly")]
	let discriminant = quote! {
		let __self_disc = ::core::intrinsics::discriminant_value(self);
		let __other_disc = ::core::intrinsics::discriminant_value(__other);
	};
	#[cfg(not(feature = "nightly"))]
	let discriminant = quote! {
		let __self_disc = ::core::mem::discriminant(self);
		let __other_disc = ::core::mem::discriminant(__other);
	};
	#[cfg(feature = "nightly")]
	let partial_ord = quote! {
		::core::cmp::PartialOrd::partial_cmp(&__self_disc, &__other_disc)
	};
	#[cfg(not(any(feature = "nightly", feature = "safe")))]
	let partial_ord = quote! {
		::core::cmp::PartialOrd::partial_cmp(
			&unsafe { ::core::mem::transmute::<_, isize>(__self_disc) },
			&unsafe { ::core::mem::transmute::<_, isize>(__other_disc) },
		)
	};
	#[cfg(all(not(feature = "nightly"), feature = "safe"))]
	let partial_ord = quote! {
		match self {
			Test::A { field: ref __field } => ::core::option::Option::Some(::core::cmp::Ordering::Less),
			Test::B { } =>
				match __other {
					Test::A { .. } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
					_ => ::core::option::Option::Some(::core::cmp::Ordering::Less),
				},
			Test::C(ref __0) =>
				match __other {
					Test::A { .. } | Test::B { .. } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
					_ => ::core::option::Option::Some(::core::cmp::Ordering::Less),
				},
			Test::D() =>
				match __other {
					Test::A { .. } | Test::B { .. } | Test::C(..) => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
					_ => ::core::option::Option::Some(::core::cmp::Ordering::Less),
				},
			Test::E => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
		}
	};

	test_derive(
		quote! {
			#[derive_where(PartialOrd)]
			enum Test<T> {
				A { field: std::marker::PhatomData<T>},
				B { },
				C(std::marker::PhatomData<T>),
				D(),
				E,
			}
		},
		quote! {
			impl<T> ::core::cmp::PartialOrd for Test<T> {
				#[inline]
				fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
					#discriminant

					if __self_disc == __other_disc {
						match (self, __other) {
							(Test::A { field: ref __field }, Test::A { field: ref __other_field }) =>
								match ::core::cmp::PartialOrd::partial_cmp(__field, __other_field) {
									::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
									__cmp => __cmp,
								},
							(Test::C(ref __0), Test::C(ref __other_0)) =>
								match ::core::cmp::PartialOrd::partial_cmp(__0, __other_0) {
									::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
									__cmp => __cmp,
								},
							_ => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
						}
					} else {
						#partial_ord
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
			#[derive_where(Clone, Copy)]
			union Test<T> {
				a: std::marker::PhantomData<T>,
				b: u8,
			}
		},
		quote! {
			impl<T> ::core::clone::Clone for Test<T> {
				#[inline]
				fn clone(&self) -> Self {
					*self
				}
			}

			impl<T> ::core::marker::Copy for Test<T>
			{ }
		},
	)
}

#[test]
fn bound() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Ord; T)]
			#[derive_where(PartialOrd)]
			struct Test<T, U>(T, std::marker::PhantomData<U>);
		},
		quote! {
			impl<T, U> ::core::cmp::Ord for Test<T, U>
			where T: ::core::cmp::Ord
			{
				#[inline]
				fn cmp(&self, __other: &Self) -> ::core::cmp::Ordering {
					match (self, __other) {
						(Test(ref __0, ref __1), Test(ref __other_0, ref __other_1)) =>
							match ::core::cmp::Ord::cmp(__0, __other_0) {
								::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(__1, __other_1) {
									::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
									__cmp => __cmp,
								},
								__cmp => __cmp,
							},
					}
				}
			}

			impl<T, U> ::core::cmp::PartialOrd for Test<T, U> {
				#[inline]
				fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
					match (self, __other) {
						(Test(ref __0, ref __1), Test(ref __other_0, ref __other_1)) =>
							match ::core::cmp::PartialOrd::partial_cmp(__0, __other_0) {
								::core::option::Option::Some(::core::cmp::Ordering::Equal) => match ::core::cmp::PartialOrd::partial_cmp(__1, __other_1) {
									::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
									__cmp => __cmp,
								},
								__cmp => __cmp,
							},
					}
				}
			}
		},
	)
}
