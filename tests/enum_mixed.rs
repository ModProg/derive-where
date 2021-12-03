#![allow(clippy::clone_on_copy)]

#[macro_use]
mod util;

use std::{
	collections::hash_map::DefaultHasher,
	hash::{Hash, Hasher},
};

use derive_where::DeriveWhere;

use self::util::{
	AssertClone, AssertCopy, AssertDebug, AssertEq, AssertHash, AssertOrd, AssertPartialEq,
	AssertPartialOrd, Wrapper,
};

#[test]
fn all() {
	#[derive(DeriveWhere)]
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

	let test_b_1 = Test::B(42.into());
	let test_b_2 = Test::B(42.into());

	let test_c_1 = Test::C;
	let test_c_2 = Test::C;

	let assert = |test| {
		let _ = AssertClone(test);
		let _ = AssertCopy(test);
		let _ = AssertDebug(test);
		let _ = AssertEq(test);
		let _ = AssertHash(test);
		let _ = AssertOrd(test);
		let _ = AssertPartialEq(test);
		let _ = AssertPartialOrd(test);
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

	let hash = |test_1: Test<_>, test_2: Test<_>| {
		let mut hasher = DefaultHasher::new();
		test_1.hash(&mut hasher);
		let hash_1 = hasher.finish();
		let mut hasher = DefaultHasher::new();
		test_2.hash(&mut hasher);
		let hash_2 = hasher.finish();
		assert_eq!(hash_1, hash_2);
	};
	hash(test_a_1, test_a_2);
	hash(test_b_1, test_b_2);
	hash(test_c_1, test_c_2);

	assert!(test_a_1 == test_a_2);
	assert!(test_a_1 != Test::A { a: 43.into() });
	assert!(test_a_1 != Test::B(42.into()));
	assert!(test_a_1 != Test::C);
	assert!(test_b_1 == test_b_2);
	assert!(test_b_1 != Test::A { a: 42.into() });
	assert!(test_b_1 != Test::B(43.into()));
	assert!(test_b_1 != Test::C);
	assert!(test_c_1 == test_c_2);
	assert!(test_c_1 != Test::A { a: 42.into() });
	assert!(test_a_1 != Test::B(42.into()));

	assert!(test_a_1 > Test::A { a: 41.into() });
	assert!(test_a_1 < Test::A { a: 43.into() });
	assert!(test_a_1 < Test::B(42.into()));
	assert!(test_a_1 < Test::C);
	assert!(test_b_1 > Test::B(41.into()));
	assert!(test_b_1 < Test::B(43.into()));
	assert!(test_b_1 > Test::A { a: 42.into() });
	assert!(test_b_1 < Test::C);
	assert!(test_c_1 == Test::C);
	assert!(test_c_1 > Test::A { a: 42.into() });
	assert!(test_c_1 > Test::B(42.into()));
}
