use quote::quote;
use syn::Result;

use super::test_derive;

#[test]
fn clone() -> Result<()> {
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
		::core::cmp::PartialOrd::partial_cmp(&(self.clone() as isize), &(__other.clone() as isize))
	};

	test_derive(
		quote! {
			#[derive_where(Clone, PartialOrd)]
			enum Test {
				A,
				B,
				#[derive_where(incomparable)]
				C,
			}
		},
		quote! {
			impl ::core::clone::Clone for Test {
				#[inline]
				fn clone(&self) -> Self {
					match self {
						Test::A => Test::A,
						Test::B => Test::B,
						Test::C => Test::C,
					}
				}
			}

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
fn copy() -> Result<()> {
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
		::core::cmp::PartialOrd::partial_cmp(&(*self as isize), &(*__other as isize))
	};

	test_derive(
		quote! {
			#[derive_where(Copy, PartialOrd)]
			enum Test {
				A,
				B,
				#[derive_where(incomparable)]
				C,
			}
		},
		quote! {
			impl ::core::marker::Copy for Test { }

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
				Test::D => (2) + 1
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
				Test::C => (3) + 1,
				Test::E => (3) + 2
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
fn repr_clone() -> Result<()> {
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
		::core::cmp::PartialOrd::partial_cmp(&(self.clone() as isize), &(__other.clone() as isize))
	};

	test_derive(
		quote! {
			#[derive_where(Clone, PartialOrd)]
			#[repr(u64)]
			enum Test {
				A,
				B,
				#[derive_where(incomparable)]
				C,
			}
		},
		quote! {
			impl ::core::clone::Clone for Test {
				#[inline]
				fn clone(&self) -> Self {
					match self {
						Test::A => Test::A,
						Test::B => Test::B,
						Test::C => Test::C,
					}
				}
			}

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
fn repr_copy() -> Result<()> {
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
		::core::cmp::PartialOrd::partial_cmp(&(*self as isize), &(*__other as isize))
	};

	test_derive(
		quote! {
			#[derive_where(Copy, PartialOrd)]
			#[repr(u64)]
			enum Test {
				A,
				B,
				#[derive_where(incomparable)]
				C,
			}
		},
		quote! {
			impl ::core::marker::Copy for Test { }

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
				Test::D => (2) + 1
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
				Test::C => (3) + 1,
				Test::E => (3) + 2
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
fn repr_expr() -> Result<()> {
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
				Test::A => u64::MAX - 2,
				Test::B => (u64::MAX - 2) + 1,
				Test::C => (u64::MAX - 2) + 2
			}
		}

		::core::cmp::PartialOrd::partial_cmp(&__discriminant(self), &__discriminant(__other))
	};

	test_derive(
		quote! {
			#[derive_where(PartialOrd)]
			#[repr(u64)]
			enum Test {
				A = u64::MAX - 2,
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