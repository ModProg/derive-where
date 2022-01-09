#![cfg(feature = "zeroize")]

extern crate zeroize_ as zeroize;

mod util;

use std::{
	marker::PhantomData,
	mem,
	ops::{Deref, DerefMut},
};

use derive_where::DeriveWhere;
use zeroize::Zeroize;

#[cfg(feature = "zeroize-on-drop")]
use self::util::AssertZeroizeOnDrop;
use self::util::{AssertZeroize, Wrapper};

#[test]
fn basic() {
	#[derive(DeriveWhere)]
	#[derive_where(Zeroize)]
	struct Test<T>(Wrapper<T>);

	let mut test = Test(42.into());

	let _ = AssertZeroize(&test);

	test.zeroize();

	assert_eq!(test.0, 0);

	util::test_drop(Test(42.into()), |test| assert_eq!(test.0, 42))
}

#[test]
fn crate_() {
	#[derive(DeriveWhere)]
	#[derive_where(Zeroize(crate = "zeroize_"))]
	struct Test<T>(Wrapper<T>);

	let mut test = Test(42.into());

	let _ = AssertZeroize(&test);

	test.zeroize();

	assert_eq!(test.0, 0);

	util::test_drop(Test(42.into()), |test| assert_eq!(test.0, 42))
}

#[test]
fn drop() {
	#[derive(DeriveWhere)]
	#[derive_where(Zeroize, ZeroizeOnDrop)]
	struct Test<T>(Wrapper<T>);

	let mut test = Test(42.into());

	let _ = AssertZeroize(&test);
	#[cfg(feature = "zeroize-on-drop")]
	let _ = AssertZeroizeOnDrop(&test);
	assert!(mem::needs_drop::<Test<()>>());

	test.zeroize();

	assert_eq!(test.0, 0);

	util::test_drop(Test(42.into()), |test| assert_eq!(test.0, 0))
}

#[test]
fn fqs() {
	struct Fqs<T>(Wrapper<T>);

	impl<T> Zeroize for Fqs<T> {
		fn zeroize(&mut self) {
			self.0.zeroize()
		}
	}

	impl<T> Fqs<T> {
		#[allow(dead_code)]
		fn zeroize(&mut self) {
			unimplemented!()
		}
	}

	#[derive(DeriveWhere)]
	#[derive_where(Zeroize, ZeroizeOnDrop)]
	struct Test<T>(#[derive_where(Zeroize(fqs))] Fqs<T>);

	let mut test = Test(Fqs(42.into()));

	let _ = AssertZeroize(&test);
	#[cfg(feature = "zeroize-on-drop")]
	let _ = AssertZeroizeOnDrop(&test);
	assert!(mem::needs_drop::<Test<()>>());

	test.zeroize();

	assert_eq!(test.0 .0, 0);

	util::test_drop(Test(Fqs(42.into())), |test| assert_eq!(test.0 .0, 0))
}

#[test]
fn deref() {
	struct ZeroizeDeref<T>(i32, PhantomData<T>);

	impl<T> Deref for ZeroizeDeref<T> {
		type Target = i32;

		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}

	impl<T> DerefMut for ZeroizeDeref<T> {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}

	#[derive(DeriveWhere)]
	#[derive_where(Zeroize, ZeroizeOnDrop)]
	struct Test<T>(ZeroizeDeref<T>);

	let mut test = Test::<()>(ZeroizeDeref(42, PhantomData));

	let _ = AssertZeroize(&test);
	#[cfg(feature = "zeroize-on-drop")]
	let _ = AssertZeroizeOnDrop(&test);
	assert!(mem::needs_drop::<Test<()>>());

	test.zeroize();

	assert_eq!(test.0 .0, 0);

	util::test_drop(Test::<()>(ZeroizeDeref(42, PhantomData)), |test| {
		assert_eq!(test.0 .0, 0)
	})
}

mod hygiene {
	use derive_where::DeriveWhere;

	use crate::util::{AssertZeroize, Wrapper};

	trait Zeroize {
		fn zeroize(&mut self) {
			unimplemented!()
		}
	}

	impl<T: zeroize::Zeroize> Zeroize for T {}

	#[test]
	fn hygiene() {
		#[derive(DeriveWhere)]
		#[derive_where(Zeroize)]
		struct Test<T>(#[derive_where(Zeroize(fqs))] Wrapper<T>);

		impl<T> Test<T> {
			#[allow(dead_code)]
			fn zeroize(&mut self) {
				unimplemented!()
			}
		}

		let mut test = Test(42.into());

		let _ = AssertZeroize(&test);

		zeroize::Zeroize::zeroize(&mut test);

		assert_eq!(test.0, 0);
	}
}
