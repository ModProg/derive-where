#![allow(unused)]
use std::cmp::Ordering;

use derive_where::derive_where;
use pretty_assertions::assert_eq;

use crate::util::{
	self, AssertClone, AssertDebug, AssertHash, AssertOrd, AssertPartialEq, AssertPartialOrd,
	Wrapper,
};

#[test]
fn debug() {
	#[derive_where(Debug)]
	enum Test<T> {
		#[derive_where(skip_inner(Debug))]
		A(Wrapper<T>),
	}

	let test_1 = Test::A(42.into());

	let _ = AssertDebug(&test_1);

	assert_eq!(format!("{:?}", test_1), "A");
}

#[test]
fn clone() {
	#[derive_where(Clone)]
	enum Test<T> {
		#[derive_where(skip_inner(Clone))]
		A(Wrapper<T>),
	}

	let test_1 = Test::A(42.into());

	let _ = AssertClone(&test_1);

	let Test::A(cloned) = test_1.clone();
	assert_eq!(cloned, 0);
}

#[test]
fn hash() {
	#[derive_where(Hash)]
	enum Test<T> {
		#[derive_where(skip_inner(Hash))]
		A(Wrapper<T>),
	}

	let test_1 = Test::A(42.into());
	let test_2 = Test::A(42.into());
	let test_ge = Test::A(43.into());

	let _ = AssertHash(&test_1);

	util::hash_eq(&test_1, &test_2);
	util::hash_eq(test_1, test_ge);
}

#[test]
fn ord() {
	#[derive_where(Eq, Ord, PartialEq, PartialOrd)]
	enum Test<T> {
		#[derive_where(skip_inner(EqHashOrd))]
		A(Wrapper<T>),
	}

	let test_1 = Test::A(42.into());
	let test_2 = Test::A(42.into());
	let test_le = Test::A(43.into());
	let test_ge = Test::A(43.into());

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
	#[derive_where(Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Clone)]
	enum Test<T> {
		#[derive_where(skip_inner(Debug, EqHashOrd, Clone))]
		A(Wrapper<T>),
	}

	let test_1 = Test::A(42.into());
	let test_2 = Test::A(42.into());
	let test_le = Test::A(41.into());
	let test_ge = Test::A(43.into());

	let _ = AssertDebug(&test_1);
	let _ = AssertClone(&test_1);
	let _ = AssertHash(&test_1);
	let _ = AssertOrd(&test_1);
	let _ = AssertPartialEq(&test_1);
	let _ = AssertPartialOrd(&test_1);

	assert_eq!(format!("{:?}", test_1), "A");

	assert_eq!(test_1.clone(), Test::A(0.into()));

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
