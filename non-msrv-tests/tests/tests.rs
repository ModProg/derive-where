#![allow(dead_code)]

#[cfg(feature = "zeroize")]
extern crate zeroize_ as zeroize;

use derive_where::derive_where;

#[test]
#[cfg(feature = "zeroize")]
fn test_zeroize() {
    use crate::zeroize::Zeroize;

    #[derive_where(Zeroize; T)]
    struct Test1<T>(T);

    let mut test = Test1(42);
    test.zeroize();

    assert_eq!(test.0, 0);

    #[derive_where(Zeroize(crate = "zeroize_"); T)]
    struct Test2<T>(T);

    let mut test = Test2(42);
    test.zeroize();

    assert_eq!(test.0, 0);
}
