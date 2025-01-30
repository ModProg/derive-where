#![allow(unused)]
use std::cmp::Ordering;

use derive_where::derive_where;

use crate::util::{
	self, AssertClone, AssertDebug, AssertHash, AssertOrd, AssertPartialEq, AssertPartialOrd,
	Wrapper,
};

#[test]
fn debug() {
	#[derive_where(Debug)]
	struct Test<T>(#[derive_where(skip(Debug))] Wrapper<T>);

	let test_1 = Test(42.into());

	let _ = AssertDebug(&test_1);

	assert_eq!(format!("{:?}", test_1), "Test");
}

#[test]
fn clone() {
	#[derive_where(Clone)]
	struct Test<T>(#[derive_where(skip(Clone))] Wrapper<T>);

	let test_1 = Test(42.into());

	let _ = AssertClone(&test_1);

	assert_eq!(test_1.clone().0, 0)
}

#[test]
fn hash() {
	#[derive_where(Hash)]
	struct Test<T>(#[derive_where(skip(Hash))] Wrapper<T>);

	let test_1 = Test(42.into());
	let test_2 = Test(42.into());
	let test_ge = Test(43.into());

	let _ = AssertHash(&test_1);

	util::hash_eq(&test_1, &test_2);
	util::hash_eq(test_1, test_ge);
}

#[test]
fn ord() {
	#[derive_where(Eq, Ord, PartialEq, PartialOrd)]
	struct Test<T>(#[derive_where(skip(EqHashOrd))] Wrapper<T>);

	let test_1 = Test(42.into());
	let test_2 = Test(42.into());
	let test_le = Test(43.into());
	let test_ge = Test(43.into());

	let _ = AssertOrd(&test_1);

	assert_eq!(test_1.cmp(&test_2), Ordering::Equal);
	assert_eq!(test_1.cmp(&test_le), Ordering::Equal);
	assert_eq!(test_1.cmp(&test_ge), Ordering::Equal);

	let _ = AssertPartialEq(&test_1);

	assert!(test_1 == test_2);
	assert!(test_1 == test_ge);

	let _ = AssertPartialOrd(&test_1);

	assert_eq!(test_1.partial_cmp(&test_2), Some(Ordering::Equal));
	assert_eq!(test_1.partial_cmp(&test_le), Some(Ordering::Equal));
	assert_eq!(test_1.partial_cmp(&test_ge), Some(Ordering::Equal));
}

#[test]
fn all() {
	#[derive_where(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
	struct Test<T>(#[derive_where(skip(Debug, EqHashOrd))] Wrapper<T>);

	let test_1 = Test(42.into());
	let test_2 = Test(42.into());
	let test_le = Test(41.into());
	let test_ge = Test(43.into());

	let _ = AssertDebug(&test_1);
	let _ = AssertHash(&test_1);
	let _ = AssertOrd(&test_1);
	let _ = AssertPartialEq(&test_1);
	let _ = AssertPartialOrd(&test_1);

	assert_eq!(format!("{:?}", test_1), "Test");

	util::hash_eq(&test_1, &test_2);
	util::hash_eq(&test_1, &test_ge);

	assert_eq!(test_1.cmp(&test_2), Ordering::Equal);
	assert_eq!(test_1.cmp(&test_le), Ordering::Equal);
	assert_eq!(test_1.cmp(&test_ge), Ordering::Equal);

	assert!(test_1 == test_2);
	assert!(test_1 == test_ge);

	assert_eq!(test_1.partial_cmp(&test_2), Some(Ordering::Equal));
	assert_eq!(test_1.partial_cmp(&test_le), Some(Ordering::Equal));
	assert_eq!(test_1.partial_cmp(&test_ge), Some(Ordering::Equal));
}
