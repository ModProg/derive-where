#![allow(clippy::clone_on_copy)]

mod util;

use std::{
	cmp::Ordering,
	fmt::{self, Formatter},
	hash::Hasher,
};

use derive_where::DeriveWhere;

use self::util::{
	AssertClone, AssertCopy, AssertDebug, AssertEq, AssertHash, AssertOrd, AssertPartialEq,
	AssertPartialOrd, Wrapper,
};

trait Clone {
	fn clone(&self) -> Self;
}

impl<T: std::clone::Clone> Clone for T {
	fn clone(&self) -> Self {
		unimplemented!()
	}
}

trait Debug {
	fn fmt(&self, _: Formatter) -> fmt::Result {
		unimplemented!()
	}
}

impl<T: std::fmt::Debug> Debug for T {}

trait Default {
	fn default() -> Self;
}

impl<T: std::default::Default> Default for T {
	fn default() -> Self {
		unimplemented!()
	}
}

trait Hash {
	fn hash<H>(&self, _: &mut H)
	where
		H: Hasher,
	{
		unimplemented!()
	}
}

impl<T: std::hash::Hash> Hash for T {}

trait Ord: Eq + PartialOrd<Self> {
	fn cmp(&self, _: &Self) -> Ordering {
		unimplemented!()
	}
}

impl<T: std::cmp::Ord> Ord for T {}

trait PartialEq<Rhs = Self>
where
	Rhs: ?Sized,
{
	fn eq(&self, _: &Rhs) -> bool {
		unimplemented!()
	}
}

impl<T: std::cmp::PartialEq> PartialEq for T {}

trait PartialOrd<Rhs = Self>: PartialEq<Rhs>
where
	Rhs: ?Sized,
{
	fn partial_cmp(&self, _: &Rhs) -> Option<Ordering> {
		unimplemented!()
	}
}

impl<T: std::cmp::PartialOrd> PartialOrd for T {}

#[test]
fn test() {
	#[derive(DeriveWhere)]
	#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
	struct Test<T>(Wrapper<T>);

	let test_1 = Test(42.into());
	let test_2 = Test(42.into());
	let test_le = Test(41.into());
	let test_ge = Test(43.into());

	let _ = AssertClone(&test_1);
	let _ = AssertCopy(&test_1);
	let _ = AssertDebug(&test_1);
	let _ = AssertEq(&test_1);
	let _ = AssertHash(&test_1);
	let _ = AssertOrd(&test_1);
	let _ = AssertPartialEq(&test_1);
	let _ = AssertPartialOrd(&test_1);

	let test_clone = std::clone::Clone::clone(&test_1);
	assert_eq!(test_clone.0, 42);

	let test_copy = test_1;
	assert_eq!(test_copy.0, 42);

	assert_eq!(format!("{:?}", test_1), "Test(42)");

	let test_default = <Test<i32> as std::default::Default>::default();
	assert_eq!(test_default.0, <i32 as std::default::Default>::default());

	util::hash_eq(test_1, test_2);
	util::hash_ne(test_1, test_ge);

	assert!(test_1 == test_2);
	assert!(test_1 != test_ge);

	assert_eq!(std::cmp::Ord::cmp(&test_1, &test_2), Ordering::Equal);
	assert_eq!(std::cmp::Ord::cmp(&test_1, &test_le), Ordering::Greater);
	assert_eq!(std::cmp::Ord::cmp(&test_1, &test_ge), Ordering::Less);

	assert_eq!(
		std::cmp::PartialOrd::partial_cmp(&test_1, &test_2),
		Some(Ordering::Equal)
	);
	assert!(test_1 > Test(41.into()));
	assert!(test_1 < test_ge);
}
