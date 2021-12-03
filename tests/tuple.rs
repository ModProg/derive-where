#![allow(clippy::clone_on_copy)]

mod util;

use core::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use derive_where::DeriveWhere;

use self::util::{
	AssertClone, AssertCopy, AssertDebug, AssertEq, AssertHash, AssertOrd, AssertPartialEq,
	AssertPartialOrd, Wrapper,
};

#[test]
fn single() {
	#[derive(DeriveWhere)]
	#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
	struct Test<T>(Wrapper<T>);

	let test_1 = Test(42.into());
	let test_2 = Test(42.into());

	let _ = AssertClone(test_1);
	let _ = AssertCopy(test_1);
	let _ = AssertDebug(test_1);
	let _ = AssertEq(test_1);
	let _ = AssertHash(test_1);
	let _ = AssertOrd(test_1);
	let _ = AssertPartialEq(test_1);
	let _ = AssertPartialOrd(test_1);

	let test_clone = test_1.clone();
	assert_eq!(test_clone.0, 42);

	let test_copy = test_1;
	assert_eq!(test_copy.0, 42);

	assert_eq!(format!("{:?}", test_1), "Test(42)");

	let test_default = Test::<i32>::default();
	assert_eq!(test_default.0, i32::default());

	let mut hasher = DefaultHasher::new();
	test_1.hash(&mut hasher);
	let hash_1 = hasher.finish();
	let mut hasher = DefaultHasher::new();
	test_2.hash(&mut hasher);
	let hash_2 = hasher.finish();
	assert_eq!(hash_1, hash_2);

	assert!(test_1 == test_2);
	assert!(test_1 != Test(43.into()));

	assert!(test_1 > Test(41.into()));
	assert!(test_1 < Test(43.into()));
}

#[test]
fn multiple() {
	#[derive(DeriveWhere)]
	#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
	struct Test<T>(Wrapper<T>, Wrapper<T>, Wrapper<T>);

	let test_1 = Test(42.into(), 43.into(), 44.into());
	let test_2 = Test(42.into(), 43.into(), 44.into());

	let _ = AssertClone(test_1);
	let _ = AssertCopy(test_1);
	let _ = AssertDebug(test_1);
	let _ = AssertEq(test_1);
	let _ = AssertHash(test_1);
	let _ = AssertOrd(test_1);
	let _ = AssertPartialEq(test_1);
	let _ = AssertPartialOrd(test_1);

	let test_clone = test_1.clone();
	assert_eq!(test_clone.0, 42);
	assert_eq!(test_clone.1, 43);
	assert_eq!(test_clone.2, 44);

	let test_copy = test_1;
	assert_eq!(test_copy.0, 42);
	assert_eq!(test_copy.1, 43);
	assert_eq!(test_copy.2, 44);

	assert_eq!(format!("{:?}", test_1), "Test(42, 43, 44)");

	let test_default = Test::<i32>::default();
	assert_eq!(test_default.0, i32::default());
	assert_eq!(test_default.1, i32::default());
	assert_eq!(test_default.2, i32::default());

	let mut hasher = DefaultHasher::new();
	test_1.hash(&mut hasher);
	let hash_1 = hasher.finish();
	let mut hasher = DefaultHasher::new();
	test_2.hash(&mut hasher);
	let hash_2 = hasher.finish();
	assert_eq!(hash_1, hash_2);

	assert!(test_1 == test_2);
	assert!(test_1 != Test(43.into(), 43.into(), 44.into(),));
	assert!(test_1 != Test(42.into(), 44.into(), 44.into()));
	assert!(test_1 != Test(42.into(), 43.into(), 45.into()));
	assert!(test_1 != Test(45.into(), 45.into(), 45.into()));

	assert!(test_1 > Test(41.into(), 43.into(), 44.into()));
	assert!(test_1 > Test(42.into(), 42.into(), 44.into()));
	assert!(test_1 > Test(42.into(), 43.into(), 43.into()));
	assert!(test_1 < Test(43.into(), 43.into(), 44.into()));
	assert!(test_1 < Test(42.into(), 44.into(), 44.into()));
	assert!(test_1 < Test(42.into(), 43.into(), 45.into()));
}
