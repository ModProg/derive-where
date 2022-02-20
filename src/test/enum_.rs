use quote::quote;
use syn::Result;

use super::test_derive;

#[test]
fn default_struct() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Default; T)]
			enum Test<T> {
				#[derive_where(default)]
				A { field: T },
			}
		},
		quote! {
			impl<T> ::core::default::Default for Test<T>
			where T: ::core::default::Default
			{
				fn default() -> Self {
					Test::A { field: ::core::default::Default::default() }
				}
			}
		},
	)
}

#[test]
fn default_tuple() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Default; T)]
			enum Test<T> {
				#[derive_where(default)]
				A(T),
			}
		},
		quote! {
			impl<T> ::core::default::Default for Test<T>
			where T: ::core::default::Default
			{
				fn default() -> Self {
					Test::A(::core::default::Default::default())
				}
			}
		},
	)
}

#[test]
fn default_unit() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Default; T)]
			enum Test<T> {
				#[derive_where(default)]
				A,
				B(T),
			}
		},
		quote! {
			impl<T> ::core::default::Default for Test<T>
			where T: ::core::default::Default
			{
				fn default() -> Self {
					Test::A
				}
			}
		},
	)
}

#[test]
fn one_data() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(PartialEq, PartialOrd)]
			enum Test<T> { A(std::marker::PhantomData<T>) }
		},
		quote! {
			impl<T> ::core::cmp::PartialEq for Test<T> {
				#[inline]
				fn eq(&self, __other: &Self) -> bool {
					match (self, __other) {
						(Test::A(ref __0), Test::A(ref __other_0)) =>
							true && ::core::cmp::PartialEq::eq(__0, __other_0),
					}
				}
			}

			impl<T> ::core::cmp::PartialOrd for Test<T> {
				#[inline]
				fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
					match (self, __other) {
						(Test::A(ref __0), Test::A(ref __other_0)) =>
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
fn two_data() -> Result<()> {
	#[cfg(not(feature = "safe"))]
	let unreachable = quote! { unsafe { ::core::hint::unreachable_unchecked() } };
	#[cfg(feature = "safe")]
	let unreachable = quote! { ::core::unreachable!("comparing variants yielded unexpected results") };
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
			Test::A(ref __0) => ::core::option::Option::Some(::core::cmp::Ordering::Less),
			Test::B(ref __0) => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
		}
	};

	test_derive(
		quote! {
			#[derive_where(PartialEq, PartialOrd)]
			enum Test<T> { A(std::marker::PhantomData<T>), B(std::marker::PhantomData<T>) }
		},
		quote! {
			impl<T> ::core::cmp::PartialEq for Test<T> {
				#[inline]
				fn eq(&self, __other: &Self) -> bool {
					if ::core::mem::discriminant(self) == ::core::mem::discriminant(__other) {
						match (self, __other) {
							(Test::A(ref __0), Test::A(ref __other_0)) =>
								true && ::core::cmp::PartialEq::eq(__0, __other_0),
							(Test::B(ref __0), Test::B(ref __other_0)) =>
								true && ::core::cmp::PartialEq::eq(__0, __other_0),
							_ => #unreachable,
						}
					} else {
						false
					}
				}
			}

			impl<T> ::core::cmp::PartialOrd for Test<T> {
				#[inline]
				fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
					#discriminant

					if __self_disc == __other_disc {
						match (self, __other) {
							(Test::A(ref __0), Test::A(ref __other_0)) =>
								match ::core::cmp::PartialOrd::partial_cmp(__0, __other_0) {
									::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
									__cmp => __cmp,
								},
							(Test::B(ref __0), Test::B(ref __other_0)) =>
								match ::core::cmp::PartialOrd::partial_cmp(__0, __other_0) {
									::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
									__cmp => __cmp,
								},
							_ => #unreachable,
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
fn unit() -> Result<()> {
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
			Test::A(ref __0) => ::core::option::Option::Some(::core::cmp::Ordering::Less),
			Test::B => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
		}
	};

	test_derive(
		quote! {
			#[derive_where(PartialEq, PartialOrd)]
			enum Test<T> { A(std::marker::PhantomData<T>), B }
		},
		quote! {
			impl<T> ::core::cmp::PartialEq for Test<T> {
				#[inline]
				fn eq(&self, __other: &Self) -> bool {
					if ::core::mem::discriminant(self) == ::core::mem::discriminant(__other) {
						match (self, __other) {
							(Test::A(ref __0), Test::A(ref __other_0)) =>
								true && ::core::cmp::PartialEq::eq(__0, __other_0),
							_ => true,
						}
					} else {
						false
					}
				}
			}

			impl<T> ::core::cmp::PartialOrd for Test<T> {
				#[inline]
				fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
					#discriminant

					if __self_disc == __other_disc {
						match (self, __other) {
							(Test::A(ref __0), Test::A(ref __other_0)) =>
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
fn struct_unit() -> Result<()> {
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
			Test::A(ref __0) => ::core::option::Option::Some(::core::cmp::Ordering::Less),
			Test::B { } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
		}
	};

	test_derive(
		quote! {
			#[derive_where(PartialEq, PartialOrd)]
			enum Test<T> { A(std::marker::PhantomData<T>), B { } }
		},
		quote! {
			impl<T> ::core::cmp::PartialEq for Test<T> {
				#[inline]
				fn eq(&self, __other: &Self) -> bool {
					if ::core::mem::discriminant(self) == ::core::mem::discriminant(__other) {
						match (self, __other) {
							(Test::A(ref __0), Test::A(ref __other_0)) =>
								true && ::core::cmp::PartialEq::eq(__0, __other_0),
							_ => true,
						}
					} else {
						false
					}
				}
			}

			impl<T> ::core::cmp::PartialOrd for Test<T> {
				#[inline]
				fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
					#discriminant

					if __self_disc == __other_disc {
						match (self, __other) {
							(Test::A(ref __0), Test::A(ref __other_0)) =>
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
fn tuple_unit() -> Result<()> {
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
			Test::A(ref __0) => ::core::option::Option::Some(::core::cmp::Ordering::Less),
			Test::B() => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
		}
	};

	test_derive(
		quote! {
			#[derive_where(PartialEq, PartialOrd)]
			enum Test<T> { A(std::marker::PhantomData<T>), B() }
		},
		quote! {
			impl<T> ::core::cmp::PartialEq for Test<T> {
				#[inline]
				fn eq(&self, __other: &Self) -> bool {
					if ::core::mem::discriminant(self) == ::core::mem::discriminant(__other) {
						match (self, __other) {
							(Test::A(ref __0), Test::A(ref __other_0)) =>
								true && ::core::cmp::PartialEq::eq(__0, __other_0),
							_ => true,
						}
					} else {
						false
					}
				}
			}

			impl<T> ::core::cmp::PartialOrd for Test<T> {
				#[inline]
				fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
					#discriminant

					if __self_disc == __other_disc {
						match (self, __other) {
							(Test::A(ref __0), Test::A(ref __other_0)) =>
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
