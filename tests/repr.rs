#[macro_use]
mod util;

use std::cmp::Ordering;

use derive_where::derive_where;

use self::util::{AssertPartialEq, AssertPartialOrd, Wrapper};

#[test]
fn repr() {
	#[derive_where(PartialEq, PartialOrd)]
	#[repr(u8)]
	enum Test {
		A,
		B,
		#[derive_where(incomparable)]
		C,
	}

	let test_a_1 = Test::A;
	let test_a_2 = Test::A;

	let test_b_1 = Test::B;
	let test_b_2 = Test::B;

	let test_c_1 = Test::C;

	let assert = |test| {
		let _ = AssertPartialEq(test);
		let _ = AssertPartialOrd(test);
	};
	assert(&test_a_1);
	assert(&test_b_1);
	assert(&test_c_1);

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_b_1);
	assert!(test_a_1 != test_c_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);
	assert!(test_b_1 != test_c_1);
	assert!(test_c_1 != test_a_1);
	assert!(test_a_1 != test_b_1);

	assert_eq!(test_a_1.partial_cmp(&test_a_2), Some(Ordering::Equal));
	assert!(test_a_1 < test_b_1);
	assert_eq!(test_b_1.partial_cmp(&test_b_2), Some(Ordering::Equal));
	assert!(test_b_1 > test_a_1);
}

#[test]
fn repr_with_values() {
	#[derive_where(Eq, PartialEq, PartialOrd, Ord)]
	#[repr(u8)]
	enum Test<T> {
		A(Wrapper<T>),
		B,
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

	let assert = |test| {
		let _ = AssertPartialEq(&test);
		let _ = AssertPartialOrd(&test);
	};
	assert(&test_a_1);
	assert(&test_b_1);
	assert(&test_c_1);

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_a_ge);
	assert!(test_a_1 != test_b_1);
	assert!(test_a_1 != test_c_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);
	assert!(test_b_1 != test_c_1);
	assert!(test_c_1 == test_c_2);
	assert!(test_c_1 != test_a_1);
	assert!(test_a_1 != test_b_1);

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
fn repr_with_repr_c_with_values() {
	#[derive_where(Eq, PartialEq, PartialOrd, Ord)]
	#[repr(C, u8)]
	enum Test<T> {
		A(Wrapper<T>),
		B,
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

	let assert = |test| {
		let _ = AssertPartialEq(&test);
		let _ = AssertPartialOrd(&test);
	};
	assert(&test_a_1);
	assert(&test_b_1);
	assert(&test_c_1);

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_a_ge);
	assert!(test_a_1 != test_b_1);
	assert!(test_a_1 != test_c_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);
	assert!(test_b_1 != test_c_1);
	assert!(test_c_1 == test_c_2);
	assert!(test_c_1 != test_a_1);
	assert!(test_a_1 != test_b_1);

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
