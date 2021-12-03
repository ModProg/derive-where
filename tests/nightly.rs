#![cfg(feature = "nightly")]
#![feature(core_intrinsics)]

#[test]
fn discriminant() {
	use std::intrinsics::discriminant_value;

	enum Test {
		A,
		B,
		C,
	}

	assert_eq!(0_isize, discriminant_value(&Test::A));
	assert_eq!(1_isize, discriminant_value(&Test::B));
	assert_eq!(2_isize, discriminant_value(&Test::C));
}
