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
fn single() {
	#[derive(DeriveWhere)]
	#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
	enum Test<T> {
		#[derive_where(default)]
		A(Wrapper<T>),
	}

	let test_1 = Test::A(42.into());
	let test_2 = Test::A(42.into());

	let _ = AssertClone(test_1);
	let _ = AssertCopy(test_1);
	let _ = AssertDebug(test_1);
	let _ = AssertEq(test_1);
	let _ = AssertHash(test_1);
	let _ = AssertOrd(test_1);
	let _ = AssertPartialEq(test_1);
	let _ = AssertPartialOrd(test_1);

	let test_clone = test_1.clone();
	assert!(matches!(test_clone, Test::A(a) if a == 42));

	let test_copy = test_1;
	assert!(matches!(test_copy, Test::A(a) if a == 42));

	assert_eq!(format!("{:?}", test_1), "A(42)");

	let test_default = Test::<i32>::default();
	assert!(matches!(test_default, Test::A(a) if a == i32::default()));

	let mut hasher = DefaultHasher::new();
	test_1.hash(&mut hasher);
	let hash_1 = hasher.finish();
	let mut hasher = DefaultHasher::new();
	test_2.hash(&mut hasher);
	let hash_2 = hasher.finish();
	assert_eq!(hash_1, hash_2);

	assert!(test_1 == test_2);
	assert!(test_1 != Test::A(43.into()));

	assert!(test_1 > Test::A(41.into()));
	assert!(test_1 < Test::A(43.into()));
}

#[test]
fn multiple() {
	#[derive(DeriveWhere)]
	#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
	enum Test<T> {
		#[derive_where(default)]
		A(Wrapper<T>, Wrapper<T>, Wrapper<T>),
	}

	let test_1 = Test::A(42.into(), 43.into(), 44.into());
	let test_2 = Test::A(42.into(), 43.into(), 44.into());

	let _ = AssertClone(test_1);
	let _ = AssertCopy(test_1);
	let _ = AssertDebug(test_1);
	let _ = AssertEq(test_1);
	let _ = AssertHash(test_1);
	let _ = AssertOrd(test_1);
	let _ = AssertPartialEq(test_1);
	let _ = AssertPartialOrd(test_1);

	let test_clone = test_1.clone();
	assert!(matches!(test_clone, Test::A(a, _, _) if a == 42));
	assert!(matches!(test_clone, Test::A(_, b, _) if b == 43));
	assert!(matches!(test_clone, Test::A(_, _, c) if c == 44));

	let test_copy = test_1;
	assert!(matches!(test_copy, Test::A(a, _, _) if a == 42));
	assert!(matches!(test_copy, Test::A(_, b, _) if b == 43));
	assert!(matches!(test_copy, Test::A(_, _, c) if c == 44));

	assert_eq!(format!("{:?}", test_1), "A(42, 43, 44)");

	let test_default = Test::<i32>::default();
	assert!(matches!(test_default, Test::A(a, _, _) if a == i32::default()));
	assert!(matches!(test_default, Test::A(_, b, _) if b == i32::default()));
	assert!(matches!(test_default, Test::A(_, _, c) if c == i32::default()));

	let mut hasher = DefaultHasher::new();
	test_1.hash(&mut hasher);
	let hash_1 = hasher.finish();
	let mut hasher = DefaultHasher::new();
	test_2.hash(&mut hasher);
	let hash_2 = hasher.finish();
	assert_eq!(hash_1, hash_2);

	assert!(test_1 == test_2);
	assert!(test_1 != Test::A(43.into(), 43.into(), 44.into()));
	assert!(test_1 != Test::A(42.into(), 44.into(), 44.into()));
	assert!(test_1 != Test::A(42.into(), 43.into(), 45.into()));
	assert!(test_1 != Test::A(45.into(), 45.into(), 45.into()));

	assert!(test_1 > Test::A(41.into(), 43.into(), 44.into()));
	assert!(test_1 > Test::A(42.into(), 42.into(), 44.into()));
	assert!(test_1 > Test::A(42.into(), 43.into(), 43.into()));
	assert!(test_1 < Test::A(43.into(), 43.into(), 44.into()));
	assert!(test_1 < Test::A(42.into(), 44.into(), 44.into()));
	assert!(test_1 < Test::A(42.into(), 43.into(), 45.into()));
}
