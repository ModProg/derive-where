use quote::quote;
use syn::Result;

use super::test_derive;

#[test]
fn reverse() -> Result<()> {
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
	#[cfg(not(feature = "nightly"))]
	let partial_ord = quote! {
		fn __discriminant(__this: &Test) -> isize {
			match __this {
				Test::A => 2,
				Test::B => 1,
				Test::C => 0
			}
		}

		::core::cmp::PartialOrd::partial_cmp(&__discriminant(self), &__discriminant(__other))
	};

	test_derive(
		quote! {
			#[derive_where(PartialOrd)]
			enum Test {
				A = 2,
				B = 1,
				#[derive_where(incomparable)]
				C = 0
			}
		},
		quote! {
			impl ::core::cmp::PartialOrd for Test {
				#[inline]
				fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
					if ::core::matches!(self, Test::C) || ::core::matches!(__other, Test::C) {
						return ::core::option::Option::None;
					}

					#discriminant

					if __self_disc == __other_disc {
						::core::option::Option::Some(::core::cmp::Ordering::Equal)
					} else {
						#partial_ord
					}
				}
			}
		},
	)
}

#[test]
fn mix() -> Result<()> {
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
	#[cfg(not(feature = "nightly"))]
	let partial_ord = quote! {
		fn __discriminant(__this: &Test) -> isize {
			match __this {
				Test::A => 1,
				Test::B => 0,
				Test::C => 2,
				Test::D => 3
			}
		}

		::core::cmp::PartialOrd::partial_cmp(&__discriminant(self), &__discriminant(__other))
	};

	test_derive(
		quote! {
			#[derive_where(PartialOrd)]
			enum Test {
				A = 1,
				B = 0,
				C = 2,
				#[derive_where(incomparable)]
				D
			}
		},
		quote! {
			impl ::core::cmp::PartialOrd for Test {
				#[inline]
				fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
					if ::core::matches!(self, Test::D) || ::core::matches!(__other, Test::D) {
						return ::core::option::Option::None;
					}

					#discriminant

					if __self_disc == __other_disc {
						::core::option::Option::Some(::core::cmp::Ordering::Equal)
					} else {
						#partial_ord
					}
				}
			}
		},
	)
}

#[test]
fn skip() -> Result<()> {
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
	#[cfg(not(feature = "nightly"))]
	let partial_ord = quote! {
		fn __discriminant(__this: &Test) -> isize {
			match __this {
				Test::A => 0,
				Test::B => 3,
				Test::C => 4,
				Test::E => 5
			}
		}

		::core::cmp::PartialOrd::partial_cmp(&__discriminant(self), &__discriminant(__other))
	};

	test_derive(
		quote! {
			#[derive_where(PartialOrd)]
			enum Test {
				A,
				B = 3,
				C,
				#[derive_where(incomparable)]
				E,
			}
		},
		quote! {
			impl ::core::cmp::PartialOrd for Test {
				#[inline]
				fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
					if ::core::matches!(self, Test::E) || ::core::matches!(__other, Test::E) {
						return ::core::option::Option::None;
					}

					#discriminant

					if __self_disc == __other_disc {
						::core::option::Option::Some(::core::cmp::Ordering::Equal)
					} else {
						#partial_ord
					}
				}
			}
		},
	)
}

#[test]
fn expr() -> Result<()> {
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
	#[cfg(not(feature = "nightly"))]
	let partial_ord = quote! {
		fn __discriminant(__this: &Test) -> isize {
			match __this {
				Test::A => isize::MAX - 2,
				Test::B => (isize::MAX - 2) + 1,
				Test::C => (isize::MAX - 2) + 2
			}
		}

		::core::cmp::PartialOrd::partial_cmp(&__discriminant(self), &__discriminant(__other))
	};

	test_derive(
		quote! {
			#[derive_where(PartialOrd)]
			enum Test {
				A = isize::MAX - 2,
				B,
				#[derive_where(incomparable)]
				C,
			}
		},
		quote! {
			impl ::core::cmp::PartialOrd for Test {
				#[inline]
				fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
					if ::core::matches!(self, Test::C) || ::core::matches!(__other, Test::C) {
						return ::core::option::Option::None;
					}

					#discriminant

					if __self_disc == __other_disc {
						::core::option::Option::Some(::core::cmp::Ordering::Equal)
					} else {
						#partial_ord
					}
				}
			}
		},
	)
}

#[test]
fn repr() -> Result<()> {
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
			&unsafe { *<*const _>::from(self).cast::<u64>() },
			&unsafe { *<*const _>::from(__other).cast::<u64>() },
		)
	};
	#[cfg(all(not(feature = "nightly"), feature = "safe"))]
	let partial_ord = quote! {
		fn __discriminant(__this: &Test) -> u64 {
			match __this {
				Test::A => 0,
				Test::B => 1,
				Test::C => 2
			}
		}

		::core::cmp::PartialOrd::partial_cmp(&__discriminant(self), &__discriminant(__other))
	};

	test_derive(
		quote! {
			#[derive_where(PartialOrd)]
			#[repr(u64)]
			enum Test {
				A,
				B,
				#[derive_where(incomparable)]
				C,
			}
		},
		quote! {
			impl ::core::cmp::PartialOrd for Test {
				#[inline]
				fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
					if ::core::matches!(self, Test::C) || ::core::matches!(__other, Test::C) {
						return ::core::option::Option::None;
					}

					#discriminant

					if __self_disc == __other_disc {
						::core::option::Option::Some(::core::cmp::Ordering::Equal)
					} else {
						#partial_ord
					}
				}
			}
		},
	)
}

#[test]
fn repr_reverse() -> Result<()> {
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
			&unsafe { *<*const _>::from(self).cast::<u64>() },
			&unsafe { *<*const _>::from(__other).cast::<u64>() },
		)
	};
	#[cfg(all(not(feature = "nightly"), feature = "safe"))]
	let partial_ord = quote! {
		fn __discriminant(__this: &Test) -> u64 {
			match __this {
				Test::A => 2,
				Test::B => 1,
				Test::C => 0
			}
		}

		::core::cmp::PartialOrd::partial_cmp(&__discriminant(self), &__discriminant(__other))
	};

	test_derive(
		quote! {
			#[derive_where(PartialOrd)]
			#[repr(u64)]
			enum Test {
				A = 2,
				B = 1,
				#[derive_where(incomparable)]
				C = 0,
			}
		},
		quote! {
			impl ::core::cmp::PartialOrd for Test {
				#[inline]
				fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
					if ::core::matches!(self, Test::C) || ::core::matches!(__other, Test::C) {
						return ::core::option::Option::None;
					}

					#discriminant

					if __self_disc == __other_disc {
						::core::option::Option::Some(::core::cmp::Ordering::Equal)
					} else {
						#partial_ord
					}
				}
			}
		},
	)
}

#[test]
fn repr_mix() -> Result<()> {
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
			&unsafe { *<*const _>::from(self).cast::<u64>() },
			&unsafe { *<*const _>::from(__other).cast::<u64>() },
		)
	};
	#[cfg(all(not(feature = "nightly"), feature = "safe"))]
	let partial_ord = quote! {
		fn __discriminant(__this: &Test) -> u64 {
			match __this {
				Test::A => 1,
				Test::B => 0,
				Test::C => 2,
				Test::D => 3
			}
		}

		::core::cmp::PartialOrd::partial_cmp(&__discriminant(self), &__discriminant(__other))
	};

	test_derive(
		quote! {
			#[derive_where(PartialOrd)]
			#[repr(u64)]
			enum Test {
				A = 1,
				B = 0,
				C = 2,
				#[derive_where(incomparable)]
				D,
			}
		},
		quote! {
			impl ::core::cmp::PartialOrd for Test {
				#[inline]
				fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
					if ::core::matches!(self, Test::D) || ::core::matches!(__other, Test::D) {
						return ::core::option::Option::None;
					}

					#discriminant

					if __self_disc == __other_disc {
						::core::option::Option::Some(::core::cmp::Ordering::Equal)
					} else {
						#partial_ord
					}
				}
			}
		},
	)
}

#[test]
fn repr_skip() -> Result<()> {
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
			&unsafe { *<*const _>::from(self).cast::<u64>() },
			&unsafe { *<*const _>::from(__other).cast::<u64>() },
		)
	};
	#[cfg(all(not(feature = "nightly"), feature = "safe"))]
	let partial_ord = quote! {
		fn __discriminant(__this: &Test) -> u64 {
			match __this {
				Test::A => 0,
				Test::B => 3,
				Test::C => 4,
				Test::E => 5
			}
		}

		::core::cmp::PartialOrd::partial_cmp(&__discriminant(self), &__discriminant(__other))
	};

	test_derive(
		quote! {
			#[derive_where(PartialOrd)]
			#[repr(u64)]
			enum Test {
				A,
				B = 3,
				C,
				#[derive_where(incomparable)]
				E,
			}
		},
		quote! {
			impl ::core::cmp::PartialOrd for Test {
				#[inline]
				fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
					if ::core::matches!(self, Test::E) || ::core::matches!(__other, Test::E) {
						return ::core::option::Option::None;
					}

					#discriminant

					if __self_disc == __other_disc {
						::core::option::Option::Some(::core::cmp::Ordering::Equal)
					} else {
						#partial_ord
					}
				}
			}
		},
	)
}

#[test]
fn repr_negative() -> Result<()> {
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
			&unsafe { *<*const _>::from(self).cast::<i64>() },
			&unsafe { *<*const _>::from(__other).cast::<i64>() },
		)
	};
	#[cfg(all(not(feature = "nightly"), feature = "safe"))]
	let partial_ord = quote! {
		fn __discriminant(__this: &Test) -> i64 {
			match __this {
				Test::A => -0x8000_0000_0000_0000_i64,
				Test::B => (-0x8000_0000_0000_0000_i64) + 1,
				Test::C => (-0x8000_0000_0000_0000_i64) + 2
			}
		}

		::core::cmp::PartialOrd::partial_cmp(&__discriminant(self), &__discriminant(__other))
	};

	test_derive(
		quote! {
			#[derive_where(PartialOrd)]
			#[repr(i64)]
			enum Test {
				A = -0x8000_0000_0000_0000_i64,
				B,
				#[derive_where(incomparable)]
				C,
			}
		},
		quote! {
			impl ::core::cmp::PartialOrd for Test {
				#[inline]
				fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
					if ::core::matches!(self, Test::C) || ::core::matches!(__other, Test::C) {
						return ::core::option::Option::None;
					}

					#discriminant

					if __self_disc == __other_disc {
						::core::option::Option::Some(::core::cmp::Ordering::Equal)
					} else {
						#partial_ord
					}
				}
			}
		},
	)
}
