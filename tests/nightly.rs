#![cfg(feature = "nightly")]
#![feature(core_intrinsics)]

use std::intrinsics::discriminant_value;

#[test]
fn default() {
	enum Test {
		A,
		B,
		C,
	}

	assert_eq!(0_isize, discriminant_value(&Test::A));
	assert_eq!(1_isize, discriminant_value(&Test::B));
	assert_eq!(2_isize, discriminant_value(&Test::C));
}

#[test]
fn default_with_discriminant_values() {
	enum Test {
		A = 3,
		B,
		C,
	}

	assert_eq!(3_isize, discriminant_value(&Test::A));
	assert_eq!(4_isize, discriminant_value(&Test::B));
	assert_eq!(5_isize, discriminant_value(&Test::C));
}

#[test]
fn default_with_values() {
	enum Test {
		A(u8),
		B,
		C,
	}

	assert_eq!(0_isize, discriminant_value(&Test::A(0)));
	assert_eq!(1_isize, discriminant_value(&Test::B));
	assert_eq!(2_isize, discriminant_value(&Test::C));
}

#[test]
fn repr() {
	#[repr(u8)]
	enum Test {
		A,
		B,
		C,
	}

	assert_eq!(0_u8, discriminant_value(&Test::A));
	assert_eq!(1_u8, discriminant_value(&Test::B));
	assert_eq!(2_u8, discriminant_value(&Test::C));
}

#[test]
fn repr_c() {
	#[repr(C)]
	enum Test {
		A,
		B,
		C,
	}

	assert_eq!(0_isize, discriminant_value(&Test::A));
	assert_eq!(1_isize, discriminant_value(&Test::B));
	assert_eq!(2_isize, discriminant_value(&Test::C));
}

#[test]
fn repr_c_with_values() {
	#[repr(C)]
	enum Test {
		A(u8),
		B,
		C,
	}

	assert_eq!(0_isize, discriminant_value(&Test::A(0)));
	assert_eq!(1_isize, discriminant_value(&Test::B));
	assert_eq!(2_isize, discriminant_value(&Test::C));
}
