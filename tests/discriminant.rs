#![allow(clippy::enum_clike_unportable_variant)]

#[macro_use]
mod util;

use std::cmp::Ordering;

use derive_where::derive_where;

use self::util::Wrapper;

#[test]
fn default() {
	#[derive_where(PartialEq, PartialOrd)]
	enum Test {
		A = 0,
		B,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		C,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 < test_b_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 > test_a_1);
}

#[test]
fn default_clone() {
	#[derive_where(Clone, PartialEq, PartialOrd)]
	enum Test {
		A,
		B,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		C,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 < test_b_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 > test_a_1);
}

#[test]
fn default_copy() {
	#[derive_where(Clone, Copy, PartialEq, PartialOrd)]
	enum Test {
		A,
		B,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		C,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 < test_b_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 > test_a_1);
}

#[test]
fn default_reverse() {
	#[derive_where(PartialEq, PartialOrd)]
	enum Test {
		A = 2,
		B = 1,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		C = 0,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 > test_b_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 < test_a_1);
}

#[test]
fn default_mix() {
	#[derive_where(PartialEq, PartialOrd)]
	enum Test {
		A = 1,
		B = 0,
		C = 2,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		D,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	let test_c_1 = Test::C;
	let test_c_2 = Test::C;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_a_1 != test_c_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);
	assert!(test_b_1 != test_c_1);
	assert!(test_c_1 == test_c_2);
	assert!(test_c_1 != test_a_1);
	assert!(test_c_1 != test_b_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 > test_b_1);
	assert!(test_a_1 < test_c_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 < test_a_1);
	assert!(test_b_1 < test_c_1);
	assert_eq!(test_c_1.partial_cmp(&test_c_2), Some(Ordering::Equal));
	assert!(test_c_1 > test_a_1);
	assert!(test_c_1 > test_b_1);
}

#[test]
fn default_skip() {
	#[derive_where(PartialEq, PartialOrd)]
	enum Test {
		A,
		B = 3,
		C,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		D,
		E,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	let test_c_1 = Test::C;
	let test_c_2 = Test::C;

	let test_e_1 = Test::E;
	let test_e_2 = Test::E;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_a_1 != test_c_1);
	assert!(test_a_1 != test_e_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);
	assert!(test_b_1 != test_c_1);
	assert!(test_b_1 != test_e_1);
	assert!(test_c_1 == test_c_2);
	assert!(test_c_1 != test_a_1);
	assert!(test_c_1 != test_b_1);
	assert!(test_c_1 != test_e_1);
	assert!(test_e_1 == test_e_2);
	assert!(test_e_1 != test_a_1);
	assert!(test_e_1 != test_b_1);
	assert!(test_e_1 != test_c_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 < test_b_1);
	assert!(test_a_1 < test_c_1);
	assert!(test_a_1 < test_e_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 > test_a_1);
	assert!(test_b_1 < test_c_1);
	assert!(test_b_1 < test_e_1);
	assert_eq!(test_c_1.partial_cmp(&test_c_2), Some(Ordering::Equal));
	assert!(test_c_1 > test_a_1);
	assert!(test_c_1 > test_b_1);
	assert!(test_c_1 < test_e_1);
	assert_eq!(test_e_1.partial_cmp(&test_e_2), Some(Ordering::Equal));
	assert!(test_e_1 > test_a_1);
	assert!(test_e_1 > test_b_1);
	assert!(test_e_1 > test_c_1);
}

#[test]
fn default_negative() {
	#[derive_where(PartialEq, PartialOrd)]
	enum Test {
		A = -0x8000_0000_0000_0000_isize,
		B,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		C,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 < test_b_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 > test_a_1);
}

#[test]
fn default_expr() {
	#[derive_where(PartialEq, PartialOrd)]
	enum Test {
		A = isize::MAX - 2,
		B,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		C,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 < test_b_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 > test_a_1);
}

#[test]
fn repr_c() {
	#[derive_where(PartialEq, PartialOrd)]
	#[repr(C)]
	enum Test {
		A = 0,
		B,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		C,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 < test_b_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 > test_a_1);
}

#[test]
fn repr_c_clone() {
	#[derive_where(Clone, PartialEq, PartialOrd)]
	#[repr(C)]
	enum Test {
		A,
		B,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		C,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 < test_b_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 > test_a_1);
}

#[test]
fn repr_c_copy() {
	#[derive_where(Clone, Copy, PartialEq, PartialOrd)]
	#[repr(C)]
	enum Test {
		A,
		B,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		C,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 < test_b_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 > test_a_1);
}

#[test]
fn repr_c_reverse() {
	#[derive_where(PartialEq, PartialOrd)]
	#[repr(C)]
	enum Test {
		A = 2,
		B = 1,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		C = 0,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 > test_b_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 < test_a_1);
}

#[test]
fn repr_c_mix() {
	#[derive_where(PartialEq, PartialOrd)]
	#[repr(C)]
	enum Test {
		A = 1,
		B = 0,
		C = 2,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		D,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	let test_c_1 = Test::C;
	let test_c_2 = Test::C;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_a_1 != test_c_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);
	assert!(test_b_1 != test_c_1);
	assert!(test_c_1 == test_c_2);
	assert!(test_c_1 != test_a_1);
	assert!(test_c_1 != test_b_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 > test_b_1);
	assert!(test_a_1 < test_c_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 < test_a_1);
	assert!(test_b_1 < test_c_1);
	assert_eq!(test_c_1.partial_cmp(&test_c_2), Some(Ordering::Equal));
	assert!(test_c_1 > test_a_1);
	assert!(test_c_1 > test_b_1);
}

#[test]
fn repr_c_skip() {
	#[derive_where(PartialEq, PartialOrd)]
	#[repr(C)]
	enum Test {
		A,
		B = 3,
		C,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		D,
		E,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	let test_c_1 = Test::C;
	let test_c_2 = Test::C;

	let test_e_1 = Test::E;
	let test_e_2 = Test::E;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_a_1 != test_c_1);
	assert!(test_a_1 != test_e_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);
	assert!(test_b_1 != test_c_1);
	assert!(test_b_1 != test_e_1);
	assert!(test_c_1 == test_c_2);
	assert!(test_c_1 != test_a_1);
	assert!(test_c_1 != test_b_1);
	assert!(test_c_1 != test_e_1);
	assert!(test_e_1 == test_e_2);
	assert!(test_e_1 != test_a_1);
	assert!(test_e_1 != test_b_1);
	assert!(test_e_1 != test_c_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 < test_b_1);
	assert!(test_a_1 < test_c_1);
	assert!(test_a_1 < test_e_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 > test_a_1);
	assert!(test_b_1 < test_c_1);
	assert!(test_b_1 < test_e_1);
	assert_eq!(test_c_1.partial_cmp(&test_c_2), Some(Ordering::Equal));
	assert!(test_c_1 > test_a_1);
	assert!(test_c_1 > test_b_1);
	assert!(test_c_1 < test_e_1);
	assert_eq!(test_e_1.partial_cmp(&test_e_2), Some(Ordering::Equal));
	assert!(test_e_1 > test_a_1);
	assert!(test_e_1 > test_b_1);
	assert!(test_e_1 > test_c_1);
}

#[test]
fn repr_c_negative() {
	#[derive_where(PartialEq, PartialOrd)]
	#[repr(C)]
	enum Test {
		A = -0x8000_0000_0000_0000_isize,
		B,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		C,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 < test_b_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 > test_a_1);
}

#[test]
fn repr_c_expr() {
	#[derive_where(PartialEq, PartialOrd)]
	#[repr(C)]
	enum Test {
		A = isize::MAX - 2,
		B,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		C,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 < test_b_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 > test_a_1);
}

#[rustversion::since(1.66)]
mod repr_c_with_value {
	use super::*;

	#[test]
	fn basic() {
		#[repr(C, u8)]
		#[derive_where(Eq, Ord, PartialEq, PartialOrd)]
		enum Test<T> {
			A(Wrapper<T>) = 0,
			B,
		}

		let test_a_1 = Test::A(42.into());
		let test_a_2 = Test::A(42.into());
		let test_a_le = Test::A(41.into());
		let test_a_ge = Test::A(43.into());

		let test_b_1 = Test::B;
		let test_b_2 = Test::B;

		assert!(test_a_1 == test_a_2);
		assert!(test_a_1 != test_b_1);
		assert!(test_b_1 == test_b_2);
		assert!(test_b_1 != test_a_1);

		assert_eq!(test_a_1.cmp(&test_a_2), Ordering::Equal);
		assert_eq!(test_a_1.cmp(&test_a_le), Ordering::Greater);
		assert_eq!(test_a_1.cmp(&test_a_ge), Ordering::Less);
		assert_eq!(test_a_1.cmp(&test_b_1), Ordering::Less);
		assert_eq!(test_b_1.cmp(&test_b_2), Ordering::Equal);
		assert_eq!(test_b_1.cmp(&test_a_1), Ordering::Greater);

		assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
		assert!(test_a_1 < test_b_1);
		assert!(test_a_1 > test_a_le);
		assert!(test_a_1 < test_a_ge);
		assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
		assert!(test_b_1 > test_a_1);
	}

	#[test]
	fn reverse() {
		#[repr(C, u8)]
		#[derive_where(Eq, Ord, PartialEq, PartialOrd)]
		enum Test<T> {
			A(Wrapper<T>) = 1,
			B = 0,
		}

		let test_a_1 = Test::A(42.into());
		let test_a_2 = Test::A(42.into());
		let test_a_le = Test::A(41.into());
		let test_a_ge = Test::A(43.into());

		let test_b_1 = Test::B;
		let test_b_2 = Test::B;

		assert!(test_a_1 == test_a_2);
		assert!(test_a_1 != test_b_1);
		assert!(test_b_1 == test_b_2);
		assert!(test_b_1 != test_a_1);

		assert_eq!(test_a_1.cmp(&test_a_2), Ordering::Equal);
		assert_eq!(test_a_1.cmp(&test_a_le), Ordering::Greater);
		assert_eq!(test_a_1.cmp(&test_a_ge), Ordering::Less);
		assert_eq!(test_a_1.cmp(&test_b_1), Ordering::Greater);
		assert_eq!(test_b_1.cmp(&test_b_2), Ordering::Equal);
		assert_eq!(test_b_1.cmp(&test_a_1), Ordering::Less);

		assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
		assert!(test_a_1 > test_b_1);
		assert!(test_a_1 > test_a_le);
		assert!(test_a_1 < test_a_ge);
		assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
		assert!(test_b_1 < test_a_1);
	}

	#[test]
	fn mix() {
		#[repr(C, u8)]
		#[derive_where(Eq, Ord, PartialEq, PartialOrd)]
		enum Test<T> {
			A(Wrapper<T>) = 1,
			B = 0,
			C = 2,
		}

		let test_a_1 = Test::A(42.into());
		let test_a_2 = Test::A(42.into());
		let test_a_le = Test::A(41.into());
		let test_a_ge = Test::A(43.into());

		let test_b_1 = Test::B;
		let test_b_2 = Test::B;

		let test_c_1 = Test::C;
		let test_c_2 = Test::C;

		assert!(test_a_1 == test_a_2);
		assert!(test_a_1 != test_b_1);
		assert!(test_a_1 != test_c_1);
		assert!(test_b_1 == test_b_2);
		assert!(test_b_1 != test_a_1);
		assert!(test_b_1 != test_c_1);
		assert!(test_c_1 == test_c_2);
		assert!(test_c_1 != test_a_1);
		assert!(test_c_1 != test_b_1);

		assert_eq!(test_a_1.cmp(&test_a_2), Ordering::Equal);
		assert_eq!(test_a_1.cmp(&test_a_le), Ordering::Greater);
		assert_eq!(test_a_1.cmp(&test_a_ge), Ordering::Less);
		assert_eq!(test_a_1.cmp(&test_b_1), Ordering::Greater);
		assert_eq!(test_a_1.cmp(&test_c_1), Ordering::Less);
		assert_eq!(test_b_1.cmp(&test_b_2), Ordering::Equal);
		assert_eq!(test_b_1.cmp(&test_a_1), Ordering::Less);
		assert_eq!(test_b_1.cmp(&test_c_1), Ordering::Less);
		assert_eq!(test_c_1.cmp(&test_c_2), Ordering::Equal);
		assert_eq!(test_c_1.cmp(&test_a_1), Ordering::Greater);
		assert_eq!(test_c_1.cmp(&test_b_1), Ordering::Greater);

		assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
		assert!(test_a_1 > test_a_le);
		assert!(test_a_1 < test_a_ge);
		assert!(test_a_1 > test_b_1);
		assert!(test_a_1 < test_c_1);
		assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
		assert!(test_b_1 < test_a_1);
		assert!(test_b_1 < test_c_1);
		assert_eq!(test_c_1.partial_cmp(&test_c_2), Some(Ordering::Equal));
		assert!(test_c_1 > test_a_1);
		assert!(test_c_1 > test_b_1);
	}

	#[test]
	fn skip() {
		#[repr(C, u8)]
		#[derive_where(Eq, Ord, PartialEq, PartialOrd)]
		enum Test<T> {
			A(Wrapper<T>),
			B = 3,
			C,
		}

		let test_a_1 = Test::A(42.into());
		let test_a_2 = Test::A(42.into());
		let test_a_le = Test::A(41.into());
		let test_a_ge = Test::A(43.into());

		let test_b_1 = Test::B;
		let test_b_2 = Test::B;

		let test_c_1 = Test::C;
		let test_c_2 = Test::C;

		assert!(test_a_1 == test_a_2);
		assert!(test_a_1 != test_b_1);
		assert!(test_a_1 != test_c_1);
		assert!(test_b_1 == test_b_2);
		assert!(test_b_1 != test_a_1);
		assert!(test_b_1 != test_c_1);
		assert!(test_c_1 == test_c_2);
		assert!(test_c_1 != test_a_1);
		assert!(test_c_1 != test_b_1);

		assert_eq!(test_a_1.cmp(&test_a_2), Ordering::Equal);
		assert_eq!(test_a_1.cmp(&test_a_le), Ordering::Greater);
		assert_eq!(test_a_1.cmp(&test_a_ge), Ordering::Less);
		assert_eq!(test_a_1.cmp(&test_b_1), Ordering::Less);
		assert_eq!(test_a_1.cmp(&test_c_1), Ordering::Less);
		assert_eq!(test_b_1.cmp(&test_b_2), Ordering::Equal);
		assert_eq!(test_b_1.cmp(&test_a_1), Ordering::Greater);
		assert_eq!(test_b_1.cmp(&test_c_1), Ordering::Less);
		assert_eq!(test_c_1.cmp(&test_c_2), Ordering::Equal);
		assert_eq!(test_c_1.cmp(&test_a_1), Ordering::Greater);
		assert_eq!(test_c_1.cmp(&test_b_1), Ordering::Greater);

		assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
		assert!(test_a_1 > test_a_le);
		assert!(test_a_1 < test_a_ge);
		assert!(test_a_1 < test_b_1);
		assert!(test_a_1 < test_c_1);
		assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
		assert!(test_b_1 > test_a_1);
		assert!(test_b_1 < test_c_1);
		assert_eq!(test_c_1.partial_cmp(&test_c_2), Some(Ordering::Equal));
		assert!(test_c_1 > test_a_1);
		assert!(test_c_1 > test_b_1);
	}

	#[test]
	fn negative() {
		#[repr(C, i8)]
		#[derive_where(Eq, Ord, PartialEq, PartialOrd)]
		enum Test<T> {
			A(Wrapper<T>) = -0x80_i8,
			B,
		}

		let test_a_1 = Test::A(42.into());
		let test_a_2 = Test::A(42.into());
		let test_a_le = Test::A(41.into());
		let test_a_ge = Test::A(43.into());

		let test_b_1 = Test::B;
		let test_b_2 = Test::B;

		assert!(test_a_1 == test_a_2);
		assert!(test_a_1 != test_b_1);
		assert!(test_b_1 == test_b_2);
		assert!(test_b_1 != test_a_1);

		assert_eq!(test_a_1.cmp(&test_a_2), Ordering::Equal);
		assert_eq!(test_a_1.cmp(&test_a_le), Ordering::Greater);
		assert_eq!(test_a_1.cmp(&test_a_ge), Ordering::Less);
		assert_eq!(test_a_1.cmp(&test_b_1), Ordering::Less);
		assert_eq!(test_b_1.cmp(&test_b_2), Ordering::Equal);
		assert_eq!(test_b_1.cmp(&test_a_1), Ordering::Greater);

		assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
		assert!(test_a_1 < test_b_1);
		assert!(test_a_1 > test_a_le);
		assert!(test_a_1 < test_a_ge);
		assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
		assert!(test_b_1 > test_a_1);
	}

	#[test]
	fn expr() {
		#[repr(C, u8)]
		#[derive_where(Eq, Ord, PartialEq, PartialOrd)]
		enum Test<T> {
			A(Wrapper<T>) = u8::MAX - 1,
			B,
		}

		let test_a_1 = Test::A(42.into());
		let test_a_2 = Test::A(42.into());
		let test_a_le = Test::A(41.into());
		let test_a_ge = Test::A(43.into());

		let test_b_1 = Test::B;
		let test_b_2 = Test::B;

		assert!(test_a_1 == test_a_2);
		assert!(test_a_1 != test_b_1);
		assert!(test_b_1 == test_b_2);
		assert!(test_b_1 != test_a_1);

		assert_eq!(test_a_1.cmp(&test_a_2), Ordering::Equal);
		assert_eq!(test_a_1.cmp(&test_a_le), Ordering::Greater);
		assert_eq!(test_a_1.cmp(&test_a_ge), Ordering::Less);
		assert_eq!(test_a_1.cmp(&test_b_1), Ordering::Less);
		assert_eq!(test_b_1.cmp(&test_b_2), Ordering::Equal);
		assert_eq!(test_b_1.cmp(&test_a_1), Ordering::Greater);

		assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
		assert!(test_a_1 < test_b_1);
		assert!(test_a_1 > test_a_le);
		assert!(test_a_1 < test_a_ge);
		assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
		assert!(test_b_1 > test_a_1);
	}
}

#[test]
fn repr() {
	#[derive_where(PartialEq, PartialOrd)]
	#[repr(u64)]
	enum Test {
		A = 0,
		B,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		C,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 < test_b_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 > test_a_1);
}

#[test]
fn repr_clone() {
	#[derive_where(Clone, PartialEq, PartialOrd)]
	#[repr(u64)]
	enum Test {
		A,
		B,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		C,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 < test_b_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 > test_a_1);
}

#[test]
fn repr_copy() {
	#[derive_where(Clone, Copy, PartialEq, PartialOrd)]
	#[repr(u64)]
	enum Test {
		A,
		B,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		C,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 < test_b_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 > test_a_1);
}

#[test]
fn repr_reverse() {
	#[derive_where(PartialEq, PartialOrd)]
	#[repr(u64)]
	enum Test {
		A = 2,
		B = 1,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		C = 0,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 > test_b_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 < test_a_1);
}

#[test]
fn repr_mix() {
	#[derive_where(PartialEq, PartialOrd)]
	#[repr(u64)]
	enum Test {
		A = 1,
		B = 0,
		C = 2,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		D,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	let test_c_1 = Test::C;
	let test_c_2 = Test::C;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_a_1 != test_c_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);
	assert!(test_b_1 != test_c_1);
	assert!(test_c_1 == test_c_2);
	assert!(test_c_1 != test_a_1);
	assert!(test_c_1 != test_b_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 > test_b_1);
	assert!(test_a_1 < test_c_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 < test_a_1);
	assert!(test_b_1 < test_c_1);
	assert_eq!(test_c_1.partial_cmp(&test_c_2), Some(Ordering::Equal));
	assert!(test_c_1 > test_a_1);
	assert!(test_c_1 > test_b_1);
}

#[test]
fn repr_skip() {
	#[derive_where(PartialEq, PartialOrd)]
	#[repr(u64)]
	enum Test {
		A,
		B = 3,
		C,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		D,
		E,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	let test_c_1 = Test::C;
	let test_c_2 = Test::C;

	let test_e_1 = Test::E;
	let test_e_2 = Test::E;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_a_1 != test_c_1);
	assert!(test_a_1 != test_e_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);
	assert!(test_b_1 != test_c_1);
	assert!(test_b_1 != test_e_1);
	assert!(test_c_1 == test_c_2);
	assert!(test_c_1 != test_a_1);
	assert!(test_c_1 != test_b_1);
	assert!(test_c_1 != test_e_1);
	assert!(test_e_1 == test_e_2);
	assert!(test_e_1 != test_a_1);
	assert!(test_e_1 != test_b_1);
	assert!(test_e_1 != test_c_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 < test_b_1);
	assert!(test_a_1 < test_c_1);
	assert!(test_a_1 < test_e_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 > test_a_1);
	assert!(test_b_1 < test_c_1);
	assert!(test_b_1 < test_e_1);
	assert_eq!(test_c_1.partial_cmp(&test_c_2), Some(Ordering::Equal));
	assert!(test_c_1 > test_a_1);
	assert!(test_c_1 > test_b_1);
	assert!(test_c_1 < test_e_1);
	assert_eq!(test_e_1.partial_cmp(&test_e_2), Some(Ordering::Equal));
	assert!(test_e_1 > test_a_1);
	assert!(test_e_1 > test_b_1);
	assert!(test_e_1 > test_c_1);
}

#[test]
fn repr_negative() {
	#[derive_where(PartialEq, PartialOrd)]
	#[repr(i64)]
	enum Test {
		A = -0x8000_0000_0000_0000_i64,
		B,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		C,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 < test_b_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 > test_a_1);
}

#[test]
fn repr_expr() {
	#[derive_where(PartialEq, PartialOrd)]
	#[repr(u64)]
	enum Test {
		A = u64::MAX - 2,
		B,
		#[allow(dead_code)]
		#[derive_where(incomparable)]
		C,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 < test_b_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 > test_a_1);
}

#[rustversion::since(1.66)]
mod repr_with_value {
	use super::*;

	#[test]
	fn basic() {
		#[repr(u8)]
		#[derive_where(Eq, Ord, PartialEq, PartialOrd)]
		enum Test<T> {
			A(Wrapper<T>) = 0,
			B,
		}

		let test_a_1 = Test::A(42.into());
		let test_a_2 = Test::A(42.into());
		let test_a_le = Test::A(41.into());
		let test_a_ge = Test::A(43.into());

		let test_b_1 = Test::B;
		let test_b_2 = Test::B;

		assert!(test_a_1 == test_a_2);
		assert!(test_a_1 != test_b_1);
		assert!(test_b_1 == test_b_2);
		assert!(test_b_1 != test_a_1);

		assert_eq!(test_a_1.cmp(&test_a_2), Ordering::Equal);
		assert_eq!(test_a_1.cmp(&test_a_le), Ordering::Greater);
		assert_eq!(test_a_1.cmp(&test_a_ge), Ordering::Less);
		assert_eq!(test_a_1.cmp(&test_b_1), Ordering::Less);
		assert_eq!(test_b_1.cmp(&test_b_2), Ordering::Equal);
		assert_eq!(test_b_1.cmp(&test_a_1), Ordering::Greater);

		assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
		assert!(test_a_1 < test_b_1);
		assert!(test_a_1 > test_a_le);
		assert!(test_a_1 < test_a_ge);
		assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
		assert!(test_b_1 > test_a_1);
	}

	#[test]
	fn reverse() {
		#[repr(u8)]
		#[derive_where(Eq, Ord, PartialEq, PartialOrd)]
		enum Test<T> {
			A(Wrapper<T>) = 1,
			B = 0,
		}

		let test_a_1 = Test::A(42.into());
		let test_a_2 = Test::A(42.into());
		let test_a_le = Test::A(41.into());
		let test_a_ge = Test::A(43.into());

		let test_b_1 = Test::B;
		let test_b_2 = Test::B;

		assert!(test_a_1 == test_a_2);
		assert!(test_a_1 != test_b_1);
		assert!(test_b_1 == test_b_2);
		assert!(test_b_1 != test_a_1);

		assert_eq!(test_a_1.cmp(&test_a_2), Ordering::Equal);
		assert_eq!(test_a_1.cmp(&test_a_le), Ordering::Greater);
		assert_eq!(test_a_1.cmp(&test_a_ge), Ordering::Less);
		assert_eq!(test_a_1.cmp(&test_b_1), Ordering::Greater);
		assert_eq!(test_b_1.cmp(&test_b_2), Ordering::Equal);
		assert_eq!(test_b_1.cmp(&test_a_1), Ordering::Less);

		assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
		assert!(test_a_1 > test_b_1);
		assert!(test_a_1 > test_a_le);
		assert!(test_a_1 < test_a_ge);
		assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
		assert!(test_b_1 < test_a_1);
	}

	#[test]
	fn mix() {
		#[repr(u8)]
		#[derive_where(Eq, Ord, PartialEq, PartialOrd)]
		enum Test<T> {
			A(Wrapper<T>) = 1,
			B = 0,
			C = 2,
		}

		let test_a_1 = Test::A(42.into());
		let test_a_2 = Test::A(42.into());
		let test_a_le = Test::A(41.into());
		let test_a_ge = Test::A(43.into());

		let test_b_1 = Test::B;
		let test_b_2 = Test::B;

		let test_c_1 = Test::C;
		let test_c_2 = Test::C;

		assert!(test_a_1 == test_a_2);
		assert!(test_a_1 != test_b_1);
		assert!(test_a_1 != test_c_1);
		assert!(test_b_1 == test_b_2);
		assert!(test_b_1 != test_a_1);
		assert!(test_b_1 != test_c_1);
		assert!(test_c_1 == test_c_2);
		assert!(test_c_1 != test_a_1);
		assert!(test_c_1 != test_b_1);

		assert_eq!(test_a_1.cmp(&test_a_2), Ordering::Equal);
		assert_eq!(test_a_1.cmp(&test_a_le), Ordering::Greater);
		assert_eq!(test_a_1.cmp(&test_a_ge), Ordering::Less);
		assert_eq!(test_a_1.cmp(&test_b_1), Ordering::Greater);
		assert_eq!(test_a_1.cmp(&test_c_1), Ordering::Less);
		assert_eq!(test_b_1.cmp(&test_b_2), Ordering::Equal);
		assert_eq!(test_b_1.cmp(&test_a_1), Ordering::Less);
		assert_eq!(test_b_1.cmp(&test_c_1), Ordering::Less);
		assert_eq!(test_c_1.cmp(&test_c_2), Ordering::Equal);
		assert_eq!(test_c_1.cmp(&test_a_1), Ordering::Greater);
		assert_eq!(test_c_1.cmp(&test_b_1), Ordering::Greater);

		assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
		assert!(test_a_1 > test_a_le);
		assert!(test_a_1 < test_a_ge);
		assert!(test_a_1 > test_b_1);
		assert!(test_a_1 < test_c_1);
		assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
		assert!(test_b_1 < test_a_1);
		assert!(test_b_1 < test_c_1);
		assert_eq!(test_c_1.partial_cmp(&test_c_2), Some(Ordering::Equal));
		assert!(test_c_1 > test_a_1);
		assert!(test_c_1 > test_b_1);
	}

	#[test]
	fn skip() {
		#[repr(u8)]
		#[derive_where(Eq, Ord, PartialEq, PartialOrd)]
		enum Test<T> {
			A(Wrapper<T>),
			B = 3,
			C,
		}

		let test_a_1 = Test::A(42.into());
		let test_a_2 = Test::A(42.into());
		let test_a_le = Test::A(41.into());
		let test_a_ge = Test::A(43.into());

		let test_b_1 = Test::B;
		let test_b_2 = Test::B;

		let test_c_1 = Test::C;
		let test_c_2 = Test::C;

		assert!(test_a_1 == test_a_2);
		assert!(test_a_1 != test_b_1);
		assert!(test_a_1 != test_c_1);
		assert!(test_b_1 == test_b_2);
		assert!(test_b_1 != test_a_1);
		assert!(test_b_1 != test_c_1);
		assert!(test_c_1 == test_c_2);
		assert!(test_c_1 != test_a_1);
		assert!(test_c_1 != test_b_1);

		assert_eq!(test_a_1.cmp(&test_a_2), Ordering::Equal);
		assert_eq!(test_a_1.cmp(&test_a_le), Ordering::Greater);
		assert_eq!(test_a_1.cmp(&test_a_ge), Ordering::Less);
		assert_eq!(test_a_1.cmp(&test_b_1), Ordering::Less);
		assert_eq!(test_a_1.cmp(&test_c_1), Ordering::Less);
		assert_eq!(test_b_1.cmp(&test_b_2), Ordering::Equal);
		assert_eq!(test_b_1.cmp(&test_a_1), Ordering::Greater);
		assert_eq!(test_b_1.cmp(&test_c_1), Ordering::Less);
		assert_eq!(test_c_1.cmp(&test_c_2), Ordering::Equal);
		assert_eq!(test_c_1.cmp(&test_a_1), Ordering::Greater);
		assert_eq!(test_c_1.cmp(&test_b_1), Ordering::Greater);

		assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
		assert!(test_a_1 > test_a_le);
		assert!(test_a_1 < test_a_ge);
		assert!(test_a_1 < test_b_1);
		assert!(test_a_1 < test_c_1);
		assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
		assert!(test_b_1 > test_a_1);
		assert!(test_b_1 < test_c_1);
		assert_eq!(test_c_1.partial_cmp(&test_c_2), Some(Ordering::Equal));
		assert!(test_c_1 > test_a_1);
		assert!(test_c_1 > test_b_1);
	}

	#[test]
	fn negative() {
		#[repr(i8)]
		#[derive_where(Eq, Ord, PartialEq, PartialOrd)]
		enum Test<T> {
			A(Wrapper<T>) = -0x80_i8,
			B,
		}

		let test_a_1 = Test::A(42.into());
		let test_a_2 = Test::A(42.into());
		let test_a_le = Test::A(41.into());
		let test_a_ge = Test::A(43.into());

		let test_b_1 = Test::B;
		let test_b_2 = Test::B;

		assert!(test_a_1 == test_a_2);
		assert!(test_a_1 != test_b_1);
		assert!(test_b_1 == test_b_2);
		assert!(test_b_1 != test_a_1);

		assert_eq!(test_a_1.cmp(&test_a_2), Ordering::Equal);
		assert_eq!(test_a_1.cmp(&test_a_le), Ordering::Greater);
		assert_eq!(test_a_1.cmp(&test_a_ge), Ordering::Less);
		assert_eq!(test_a_1.cmp(&test_b_1), Ordering::Less);
		assert_eq!(test_b_1.cmp(&test_b_2), Ordering::Equal);
		assert_eq!(test_b_1.cmp(&test_a_1), Ordering::Greater);

		assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
		assert!(test_a_1 < test_b_1);
		assert!(test_a_1 > test_a_le);
		assert!(test_a_1 < test_a_ge);
		assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
		assert!(test_b_1 > test_a_1);
	}

	#[test]
	fn expr() {
		#[repr(u8)]
		#[derive_where(Eq, Ord, PartialEq, PartialOrd)]
		enum Test<T> {
			A(Wrapper<T>) = u8::MAX - 1,
			B,
		}

		let test_a_1 = Test::A(42.into());
		let test_a_2 = Test::A(42.into());
		let test_a_le = Test::A(41.into());
		let test_a_ge = Test::A(43.into());

		let test_b_1 = Test::B;
		let test_b_2 = Test::B;

		assert!(test_a_1 == test_a_2);
		assert!(test_a_1 != test_b_1);
		assert!(test_b_1 == test_b_2);
		assert!(test_b_1 != test_a_1);

		assert_eq!(test_a_1.cmp(&test_a_2), Ordering::Equal);
		assert_eq!(test_a_1.cmp(&test_a_le), Ordering::Greater);
		assert_eq!(test_a_1.cmp(&test_a_ge), Ordering::Less);
		assert_eq!(test_a_1.cmp(&test_b_1), Ordering::Less);
		assert_eq!(test_b_1.cmp(&test_b_2), Ordering::Equal);
		assert_eq!(test_b_1.cmp(&test_a_1), Ordering::Greater);

		assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
		assert!(test_a_1 < test_b_1);
		assert!(test_a_1 > test_a_le);
		assert!(test_a_1 < test_a_ge);
		assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
		assert!(test_b_1 > test_a_1);
	}
}
