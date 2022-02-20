use quote::quote;
use syn::Result;

use super::test_derive;

#[test]
fn struct_inner() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Debug)]
			#[derive_where(skip_inner)]
			struct Test<T>(std::marker::PhatomData<T>);
		},
		quote! {
			impl<T> ::core::fmt::Debug for Test<T> {
				fn fmt(&self, __f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
					match self {
						Test(ref __0) => {
							let mut __builder = ::core::fmt::Formatter::debug_tuple(__f, "Test");
							::core::fmt::DebugTuple::finish(&mut __builder)
						}
					}
				}
			}
		},
	)
}

#[test]
fn enum_inner() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Debug)]
			enum Test<T> {
				#[derive_where(skip_inner)]
				A(std::marker::PhatomData<T>),
			}
		},
		quote! {
			impl<T> ::core::fmt::Debug for Test<T> {
				fn fmt(&self, __f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
					match self {
						Test::A(ref __0) => {
							let mut __builder = ::core::fmt::Formatter::debug_tuple(__f, "A");
							::core::fmt::DebugTuple::finish(&mut __builder)
						}
					}
				}
			}
		},
	)
}

#[test]
fn struct_empty() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Ord)]
			#[derive_where(skip_inner)]
			struct Test<T>(std::marker::PhatomData<T>);
		},
		quote! {
			impl<T> ::core::cmp::Ord for Test<T> {
				#[inline]
				fn cmp(&self, __other: &Self) -> ::core::cmp::Ordering {
					::core::cmp::Ordering::Equal
				}
			}
		},
	)
}

#[test]
fn variant_empty() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Ord)]
			enum Test<T> {
				#[derive_where(skip_inner)]
				A(std::marker::PhatomData<T>),
			}
		},
		quote! {
			impl<T> ::core::cmp::Ord for Test<T> {
				#[inline]
				fn cmp(&self, __other: &Self) -> ::core::cmp::Ordering {
					::core::cmp::Ordering::Equal
				}
			}
		},
	)
}

#[test]
fn variants_empty() -> Result<()> {
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
	let ord = quote! {
		::core::cmp::Ord::cmp(&__self_disc, &__other_disc)
	};
	#[cfg(not(any(feature = "nightly", feature = "safe")))]
	let ord = quote! {
		::core::cmp::Ord::cmp(
			&unsafe { ::core::mem::transmute::<_, isize>(__self_disc) },
			&unsafe { ::core::mem::transmute::<_, isize>(__other_disc) },
		)
	};

	#[cfg(all(not(feature = "nightly"), feature = "safe"))]
	let ord = quote! {
		match self {
			Test::A(ref __0) => ::core::cmp::Ordering::Less,
			Test::B(ref __0) => ::core::cmp::Ordering::Greater,
		}
	};

	test_derive(
		quote! {
			#[derive_where(Ord)]
			enum Test<T> {
				#[derive_where(skip_inner)]
				A(std::marker::PhatomData<T>),
				#[derive_where(skip_inner)]
				B(std::marker::PhatomData<T>),
			}
		},
		quote! {
			impl<T> ::core::cmp::Ord for Test<T> {
				#[inline]
				fn cmp(&self, __other: &Self) -> ::core::cmp::Ordering {
					#discriminant

					if __self_disc == __other_disc {
						::core::cmp::Ordering::Equal
					} else {
						#ord
					}
				}
			}
		},
	)
}

#[test]
fn variants_partly_empty() -> Result<()> {
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
	let ord = quote! {
		::core::cmp::Ord::cmp(&__self_disc, &__other_disc)
	};
	#[cfg(not(any(feature = "nightly", feature = "safe")))]
	let ord = quote! {
		::core::cmp::Ord::cmp(
			&unsafe { ::core::mem::transmute::<_, isize>(__self_disc) },
			&unsafe { ::core::mem::transmute::<_, isize>(__other_disc) },
		)
	};

	#[cfg(all(not(feature = "nightly"), feature = "safe"))]
	let ord = quote! {
		match self {
			Test::A(ref __0) => ::core::cmp::Ordering::Less,
			Test::B(ref __0, ref __1) => ::core::cmp::Ordering::Greater,
		}
	};

	test_derive(
		quote! {
			#[derive_where(Ord)]
			enum Test<T> {
				#[derive_where(skip_inner)]
				A(std::marker::PhatomData<T>),
				B(#[derive_where(skip)] std::marker::PhatomData<T>, std::marker::PhatomData<T>),
			}
		},
		quote! {
			impl<T> ::core::cmp::Ord for Test<T> {
				#[inline]
				fn cmp(&self, __other: &Self) -> ::core::cmp::Ordering {
					#discriminant

					if __self_disc == __other_disc {
						match (self , __other) {
							(Test::B(ref __0, ref __1), Test::B(ref __other_0, ref __other_1)) =>
								match ::core::cmp::Ord::cmp(__1 ,__other_1) {
									::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal, __cmp => __cmp,
								},
							_ => ::core::cmp::Ordering::Equal,
						}
					} else {
						#ord
					}
				}
			}
		},
	)
}
