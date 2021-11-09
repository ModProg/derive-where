#[test]
fn discriminant() {
    use core::mem::{discriminant, transmute};

    enum Test {
        A,
        B,
        C,
    }

    assert_eq!(0_isize, unsafe { transmute(discriminant(&Test::A)) });
    assert_eq!(1_isize, unsafe { transmute(discriminant(&Test::B)) });
    assert_eq!(2_isize, unsafe { transmute(discriminant(&Test::C)) });
}
