mod util;

use std::marker::PhantomData;

use derive_where::DeriveWhere;

use self::util::AssertClone;

#[test]
fn custom_generic() {
	trait Trait {
		type Type;
	}

	struct Impl;

	impl Trait for Impl {
		type Type = i32;
	}

	#[derive(DeriveWhere)]
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

	#[derive(DeriveWhere)]
	#[derive_where(Clone; T: Trait)]
	struct Test<T>(u8, PhantomData<T>);

	let test_1 = Test::<Impl>(42, PhantomData);

	let _ = AssertClone(&test_1);

	#[allow(clippy::redundant_clone)]
	let test_clone = test_1.clone();
	assert_eq!(test_clone.0, 42);
}
