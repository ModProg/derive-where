#![allow(clippy::clone_on_copy)]

#[macro_use]
mod util;

use std::cmp::Ordering;

use derive_where::derive_where;

use self::util::{
	AssertClone, AssertCopy, AssertDebug, AssertEq, AssertHash, AssertOrd, AssertPartialEq,
	AssertPartialOrd, Wrapper,
};

#[test]
fn all() {
	#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
	enum Test<T> {
		#[derive_where(default)]
		A {
			a: Wrapper<T>,
		},
		B(Wrapper<T>),
		C,
	}

	let test_a_1 = Test::A { a: 42.into() };
	let test_a_2 = Test::A { a: 42.into() };
	let test_a_le = Test::A { a: 41.into() };
	let test_a_ge = Test::A { a: 43.into() };

	let test_b_1 = Test::B(42.into());
	let test_b_2 = Test::B(42.into());
	let test_b_le = Test::B(41.into());
	let test_b_ge = Test::B(43.into());

	let test_c_1 = Test::C;
	let test_c_2 = Test::C;

	let assert = |test| {
		let _ = AssertClone(&test);
		let _ = AssertCopy(&test);
		let _ = AssertDebug(&test);
		let _ = AssertEq(&test);
		let _ = AssertHash(&test);
		let _ = AssertOrd(&test);
		let _ = AssertPartialEq(&test);
		let _ = AssertPartialOrd(&test);
	};
	assert(test_a_1);
	assert(test_b_1);
	assert(test_c_1);

	let test_clone = test_a_1.clone();
	assert!(matches!(test_clone, Test::A { a } if a == 42));
	let test_clone = test_b_1.clone();
	assert!(matches!(test_clone, Test::B(a) if a == 42));
	let test_clone = test_c_1.clone();
	assert!(matches!(test_clone, Test::C));

	let test_copy = test_a_1;
	assert!(matches!(test_copy, Test::A { a } if a == 42));
	let test_copy = test_b_1;
	assert!(matches!(test_copy, Test::B(a) if a == 42));
	let test_copy = test_c_1;
	assert!(matches!(test_copy, Test::C));

	assert_eq!(format!("{:?}", test_a_1), "A { a: 42 }");
	assert_eq!(format!("{:?}", test_b_1), "B(42)");
	assert_eq!(format!("{:?}", test_c_1), "C");

	let test_default = Test::<i32>::default();
	assert!(matches!(test_default, Test::A { a } if a == i32::default()));

	util::hash_eq(test_a_1, test_a_2);
	util::hash_eq(test_b_1, test_b_2);
	util::hash_eq(test_c_1, test_c_2);
	util::hash_ne(test_a_1, test_a_ge);
	util::hash_ne(test_b_1, test_b_ge);
	util::hash_ne(test_a_1, test_b_1);
	util::hash_ne(test_a_1, test_c_1);
	util::hash_ne(test_b_1, test_c_1);

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != test_a_ge);
	assert!(test_a_1 != test_b_1);
	assert!(test_a_1 != test_c_1);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != test_a_1);
	assert!(test_b_1 != test_b_ge);
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
	assert_eq!(test_b_1.cmp(&test_b_le), Ordering::Greater);
	assert_eq!(test_b_1.cmp(&test_b_ge), Ordering::Less);
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
	assert!(test_b_1 > test_b_le);
	assert!(test_b_1 < test_b_ge);
	assert!(test_b_1 > test_a_1);
	assert!(test_b_1 < test_c_1);
	assert_eq!(test_c_1.partial_cmp(&test_c_2), Some(Ordering::Equal));
	assert!(test_c_1 > test_a_1);
	assert!(test_c_1 > test_b_1);
}
