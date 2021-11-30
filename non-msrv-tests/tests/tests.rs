#![allow(dead_code)]

#[cfg(feature = "zeroize")]
extern crate zeroize_ as zeroize;

#[test]
#[cfg(feature = "zeroize")]
fn test_zeroize() {
	use derive_where::DeriveWhere;

	//use core::ops::{Deref, DerefMut};
	use crate::zeroize::Zeroize;

	#[derive(DeriveWhere)]
	#[derive_where(Zeroize; T)]
	struct Test1<T>(T);

	let mut test = Test1(42);
	test.zeroize();

	assert_eq!(test.0, 0);

	#[derive(DeriveWhere)]
	#[derive_where(Zeroize(crate = "zeroize_"); T)]
	struct Test2<T>(T);

	let mut test = Test2(42);
	test.zeroize();

	assert_eq!(test.0, 0);

	/*struct SliceDeref([u8; 1]);

	impl Deref for SliceDeref {
		type Target = [u8];

		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}

	impl DerefMut for SliceDeref {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}

	#[derive_where(Zeroize; T)]
	struct Test3<T>(T, SliceDeref);

	let mut test = Test3(42, SliceDeref([42]));
	test.zeroize();

	assert_eq!(test.0, 0);
	assert_eq!(test.1 .0, [0]);

	#[derive_where(Zeroize; T: DerefMut, <T as Deref>::Target: Zeroize)]
	struct Test3<T>(T);

	let mut test = Test3(SliceDeref([42]));
	test.zeroize();

	assert_eq!(test.0 .0, [0]);*/
}
