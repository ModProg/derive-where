#![allow(dead_code)]

use derive_restricted::derive_where;

// Works on my part, but units cannot have types anyways.
// #[derive_where(T: Clone; Clone)]
// struct Test<T>;

#[test]
fn test_tuple() {
    #[derive_where(T: Clone, S: Clone; Clone)]
    #[derive(Debug)]
    struct TestTuple<T, S>(T, S);

    let test = TestTuple(1, String::from("Test"));
    let cloned = test.clone();

    dbg!(test);
    dbg!(cloned);
}

#[test]
fn test_struct() {
    #[derive_where(T: Clone, S: Clone; Clone)]
    #[derive(Debug)]
    struct TestStruct<T, S> {
        a: T,
        b: S,
    }

    let test = TestStruct {
        a: 1,
        b: String::from("hi"),
    };
    let cloned = test.clone();

    dbg!(test);
    dbg!(cloned);
}

#[test]
fn test_enum() {
    #[derive_where(T: Clone, S: Clone; Clone)]
    #[derive(Debug)]
    enum TestEnum<T, S> {
        VariantStruct { field: T },
        VariantTupel(S),
        Variant,
    }
    let test = TestEnum::<u8, &str>::Variant;
    let cloned = test.clone();
    dbg!(test);
    dbg!(cloned);

    let test = TestEnum::<u8, &str>::Variant;
    let cloned = test.clone();
    dbg!(test);
    dbg!(cloned);

    let test = TestEnum::<u8, &str>::VariantTupel("hi");
    let cloned = test.clone();
    dbg!(test);
    dbg!(cloned);

    let test = TestEnum::<u8, &str>::VariantStruct { field: 8 };
    let cloned = test.clone();
    dbg!(test);
    dbg!(cloned);
}
