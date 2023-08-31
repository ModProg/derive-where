#![cfg_attr(feature = "nightly", feature(core_intrinsics))]

#[cfg(feature = "nightly")]
use std::intrinsics::discriminant_value;

#[test]
fn default() {
	enum Test {
		A,
		B,
		C,
	}

	#[cfg(feature = "nightly")]
	{
		assert_eq!(0_isize, discriminant_value(&Test::A));
		assert_eq!(1_isize, discriminant_value(&Test::B));
		assert_eq!(2_isize, discriminant_value(&Test::C));
	}
	assert_eq!(0, Test::A as isize);
	assert_eq!(1, Test::B as isize);
	assert_eq!(2, Test::C as isize);
}

#[test]
fn default_with_discriminants() {
	enum Test {
		A = 3,
		B,
		C,
	}
	assert_eq!(3, Test::A as isize);
	assert_eq!(4, Test::B as isize);
	assert_eq!(5, Test::C as isize);
}

#[test]
#[cfg(feature = "nightly")]
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

	#[cfg(feature = "nightly")]
	{
		assert_eq!(0_u8, discriminant_value(&Test::A));
		assert_eq!(1_u8, discriminant_value(&Test::B));
		assert_eq!(2_u8, discriminant_value(&Test::C));
	}
	assert_eq!(0, Test::A as u8);
	assert_eq!(1, Test::B as u8);
	assert_eq!(2, Test::C as u8);
}

#[test]
fn repr_with_discriminants() {
	#[repr(u8)]
	enum Test {
		A = 3,
		B,
		C,
	}

	#[cfg(feature = "nightly")]
	{
		assert_eq!(3_u8, discriminant_value(&Test::A));
		assert_eq!(4_u8, discriminant_value(&Test::B));
		assert_eq!(5_u8, discriminant_value(&Test::C));
	}
	assert_eq!(3, Test::A as isize);
	assert_eq!(4, Test::B as isize);
	assert_eq!(5, Test::C as isize);
}

#[test]
fn repr_with_values() {
	#[repr(u8)]
	enum Test {
		A(u8),
		B,
		C,
	}

	#[cfg(feature = "nightly")]
	{
		assert_eq!(0_u8, discriminant_value(&Test::A(0)));
		assert_eq!(1_u8, discriminant_value(&Test::B));
		assert_eq!(2_u8, discriminant_value(&Test::C));
	}
	assert_eq!(0, unsafe { *<*const _>::from(&Test::A(0)).cast::<u8>() });
	assert_eq!(1, unsafe { *<*const _>::from(&Test::B).cast::<u8>() });
	assert_eq!(2, unsafe { *<*const _>::from(&Test::C).cast::<u8>() });
}

#[test]
fn repr_with_discriminants_values() {
	#[repr(u8)]
	enum Test {
		A(u8) = 3,
		B,
		C,
	}

	#[cfg(feature = "nightly")]
	{
		assert_eq!(3_u8, discriminant_value(&Test::A(0)));
		assert_eq!(4_u8, discriminant_value(&Test::B));
		assert_eq!(5_u8, discriminant_value(&Test::C));
	}
	assert_eq!(3, unsafe { *<*const _>::from(&Test::A(0)).cast::<u8>() });
	assert_eq!(4, unsafe { *<*const _>::from(&Test::B).cast::<u8>() });
	assert_eq!(5, unsafe { *<*const _>::from(&Test::C).cast::<u8>() });
}

#[test]
fn repr_c() {
	#[repr(C)]
	enum Test {
		A,
		B,
		C,
	}

	#[cfg(feature = "nightly")]
	{
		assert_eq!(0_isize, discriminant_value(&Test::A));
		assert_eq!(1_isize, discriminant_value(&Test::B));
		assert_eq!(2_isize, discriminant_value(&Test::C));
	}
	assert_eq!(0, Test::A as isize);
	assert_eq!(1, Test::B as isize);
	assert_eq!(2, Test::C as isize);
}

#[test]
fn repr_c_with_discriminants() {
	#[repr(C)]
	enum Test {
		A = 3,
		B,
		C,
	}

	#[cfg(feature = "nightly")]
	{
		assert_eq!(3_isize, discriminant_value(&Test::A));
		assert_eq!(4_isize, discriminant_value(&Test::B));
		assert_eq!(5_isize, discriminant_value(&Test::C));
	}
	assert_eq!(3, Test::A as isize);
	assert_eq!(4, Test::B as isize);
	assert_eq!(5, Test::C as isize);
}

#[test]
fn repr_c_with_values() {
	#[repr(C)]
	enum Test {
		A(u8),
		B,
		C,
	}

	#[cfg(feature = "nightly")]
	{
		assert_eq!(0_isize, discriminant_value(&Test::A(0)));
		assert_eq!(1_isize, discriminant_value(&Test::B));
		assert_eq!(2_isize, discriminant_value(&Test::C));
	}
	assert_eq!(0, unsafe { *<*const _>::from(&Test::A(0)).cast::<isize>() });
	assert_eq!(1, unsafe { *<*const _>::from(&Test::B).cast::<isize>() });
	assert_eq!(2, unsafe { *<*const _>::from(&Test::C).cast::<isize>() });
}

#[test]
fn repr_with_repr_c_with_values() {
	#[repr(C, u8)]
	enum Test {
		A(u8),
		B,
		C,
	}

	#[cfg(feature = "nightly")]
	{
		assert_eq!(0_u8, discriminant_value(&Test::A(0)));
		assert_eq!(1_u8, discriminant_value(&Test::B));
		assert_eq!(2_u8, discriminant_value(&Test::C));
	}
	assert_eq!(0, unsafe { *<*const _>::from(&Test::A(0)).cast::<u8>() });
	assert_eq!(1, unsafe { *<*const _>::from(&Test::B).cast::<u8>() });
	assert_eq!(2, unsafe { *<*const _>::from(&Test::C).cast::<u8>() });
}

#[test]
fn repr_with_repr_c_with_values_discriminants() {
	#[repr(C, u8)]
	enum Test {
		A(u8) = 3,
		B,
		C,
	}

	#[cfg(feature = "nightly")]
	{
		assert_eq!(3_u8, discriminant_value(&Test::A(0)));
		assert_eq!(4_u8, discriminant_value(&Test::B));
		assert_eq!(5_u8, discriminant_value(&Test::C));
	}
	assert_eq!(3, unsafe { *<*const _>::from(&Test::A(0)).cast::<u8>() });
	assert_eq!(4, unsafe { *<*const _>::from(&Test::B).cast::<u8>() });
	assert_eq!(5, unsafe { *<*const _>::from(&Test::C).cast::<u8>() });
}
