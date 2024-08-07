#[test]
fn discriminant() {
	use std::mem::{discriminant, size_of, size_of_val, transmute};

	enum Test {
		A,
		B,
		C,
	}

	assert_eq!(size_of::<isize>(), size_of_val(&discriminant(&Test::A)));

	assert_eq!(0_isize, unsafe {
		transmute::<std::mem::Discriminant<Test>, isize>(discriminant(&Test::A))
	});
	assert_eq!(1_isize, unsafe {
		transmute::<std::mem::Discriminant<Test>, isize>(discriminant(&Test::B))
	});
	assert_eq!(2_isize, unsafe {
		transmute::<std::mem::Discriminant<Test>, isize>(discriminant(&Test::C))
	});
}
