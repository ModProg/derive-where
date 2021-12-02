#![allow(clippy::clone_on_copy)]

mod util;

use core::{
	fmt::Debug,
	hash::{Hash, Hasher},
};
use std::collections::hash_map::DefaultHasher;

use derive_where::DeriveWhere;

use self::util::Wrapper;

struct AssertClone<T: Clone>(T);
struct AssertCopy<T: Copy>(T);
struct AssertDebug<T: Debug>(T);
struct AssertEq<T: Eq>(T);
struct AssertHash<T: Hash>(T);
struct AssertOrd<T: Ord>(T);
struct AssertPartialEq<T: PartialEq>(T);
struct AssertPartialOrd<T: PartialOrd>(T);

#[test]
fn struct_single() {
	#[derive(DeriveWhere)]
	#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
	struct Test<T> {
		a: Wrapper<T>,
	}

	let test_1 = Test { a: 42.into() };
	let test_2 = Test { a: 42.into() };

	let _ = AssertClone(test_1);
	let _ = AssertCopy(test_1);
	let _ = AssertDebug(test_1);
	let _ = AssertEq(test_1);
	let _ = AssertHash(test_1);
	let _ = AssertOrd(test_1);
	let _ = AssertPartialEq(test_1);
	let _ = AssertPartialOrd(test_1);

	let test_clone = test_1.clone();
	assert_eq!(test_clone.a, 42);

	let test_copy = test_1;
	assert_eq!(test_copy.a, 42);

	assert_eq!(format!("{:?}", test_1), "Test { a: 42 }");

	let test_default = Test::<i32>::default();
	assert_eq!(test_default.a, i32::default());

	let mut hasher = DefaultHasher::new();
	test_1.hash(&mut hasher);
	let hash_1 = hasher.finish();
	let mut hasher = DefaultHasher::new();
	test_2.hash(&mut hasher);
	let hash_2 = hasher.finish();
	assert_eq!(hash_1, hash_2);

	assert!(test_1 == test_2);
	assert!(test_1 != Test { a: 43.into() });

	assert!(test_1 > Test { a: 41.into() });
	assert!(test_1 < Test { a: 43.into() });
}

#[test]
fn struct_multiple() {
	#[derive(DeriveWhere)]
	#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
	struct Test<T> {
		a: Wrapper<T>,
		b: Wrapper<T>,
		c: Wrapper<T>,
	}

	let test_1 = Test {
		a: 42.into(),
		b: 43.into(),
		c: 44.into(),
	};
	let test_2 = Test {
		a: 42.into(),
		b: 43.into(),
		c: 44.into(),
	};

	let _ = AssertClone(test_1);
	let _ = AssertCopy(test_1);
	let _ = AssertDebug(test_1);
	let _ = AssertEq(test_1);
	let _ = AssertHash(test_1);
	let _ = AssertOrd(test_1);
	let _ = AssertPartialEq(test_1);
	let _ = AssertPartialOrd(test_1);

	let test_clone = test_1.clone();
	assert_eq!(test_clone.a, 42);
	assert_eq!(test_clone.b, 43);
	assert_eq!(test_clone.c, 44);

	let test_copy = test_1;
	assert_eq!(test_copy.a, 42);
	assert_eq!(test_copy.b, 43);
	assert_eq!(test_copy.c, 44);

	assert_eq!(format!("{:?}", test_1), "Test { a: 42, b: 43, c: 44 }");

	let test_default = Test::<i32>::default();
	assert_eq!(test_default.a, i32::default());
	assert_eq!(test_default.b, i32::default());
	assert_eq!(test_default.c, i32::default());

	let mut hasher = DefaultHasher::new();
	test_1.hash(&mut hasher);
	let hash_1 = hasher.finish();
	let mut hasher = DefaultHasher::new();
	test_2.hash(&mut hasher);
	let hash_2 = hasher.finish();
	assert_eq!(hash_1, hash_2);

	assert!(test_1 == test_2);
	assert!(
		test_1
			!= Test {
				a: 43.into(),
				b: 43.into(),
				c: 44.into(),
			}
	);
	assert!(
		test_1
			!= Test {
				a: 42.into(),
				b: 44.into(),
				c: 44.into(),
			}
	);
	assert!(
		test_1
			!= Test {
				a: 42.into(),
				b: 43.into(),
				c: 45.into(),
			}
	);
	assert!(
		test_1
			!= Test {
				a: 45.into(),
				b: 45.into(),
				c: 45.into(),
			}
	);

	assert!(
		test_1
			> Test {
				a: 41.into(),
				b: 43.into(),
				c: 44.into(),
			}
	);
	assert!(
		test_1
			> Test {
				a: 42.into(),
				b: 42.into(),
				c: 44.into(),
			}
	);
	assert!(
		test_1
			> Test {
				a: 42.into(),
				b: 43.into(),
				c: 43.into(),
			}
	);
	assert!(
		test_1
			< Test {
				a: 43.into(),
				b: 43.into(),
				c: 44.into(),
			}
	);
	assert!(
		test_1
			< Test {
				a: 42.into(),
				b: 44.into(),
				c: 44.into(),
			}
	);
	assert!(
		test_1
			< Test {
				a: 42.into(),
				b: 43.into(),
				c: 45.into(),
			}
	);
}
