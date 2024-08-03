#![allow(unused)]
mod util;

use std::marker::PhantomData;

use derive_where::derive_where;

use self::util::{AssertClone, AssertCopy};

#[test]
fn bound() {
	#[derive_where(Clone, Copy; T)]
	struct Test<T, U>(T, std::marker::PhantomData<U>);

	let test_1 = Test(42, PhantomData::<()>);

	let _ = AssertClone(&test_1);
	let _ = AssertCopy(&test_1);

	#[allow(clippy::clone_on_copy)]
	let test_clone = test_1.clone();
	assert_eq!(test_clone.0, 42);
}

#[test]
fn custom_generic() {
	trait Trait {
		type Type;
	}

	struct Impl;

	impl Trait for Impl {
		type Type = i32;
	}

	#[derive_where(Clone; T::Type)]
	struct Test<T: Trait>(T::Type);

	let test_1 = Test::<Impl>(42);

	let _ = AssertClone(&test_1);

	#[allow(clippy::redundant_clone)]
	let test_clone = test_1.clone();
	assert_eq!(test_clone.0, 42);
}

#[test]
fn custom_bound() {
	trait Trait {}

	struct Impl;

	impl Trait for Impl {}

	#[derive_where(Clone; T: Trait)]
	struct Test<T>(u8, PhantomData<T>);

	let test_1 = Test::<Impl>(42, PhantomData);

	let _ = AssertClone(&test_1);

	#[allow(clippy::redundant_clone)]
	let test_clone = test_1.clone();
	assert_eq!(test_clone.0, 42);
}

#[test]
fn ord_requirement() {
	trait Trait {
		type Type;
	}

	struct Impl;

	impl Trait for Impl {
		type Type = i32;
	}

	#[derive_where(Eq, Ord, PartialEq, PartialOrd; T::Type)]
	struct Test<T: Trait>(T::Type);
}
