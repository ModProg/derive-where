#![allow(dead_code)]

use derive_where::derive_where;

// test macro hygiene
trait Clone {}

#[test]
fn test_() {
    trait Trait {
        type Type;
    }
    #[derive_where(T::Type; Clone, Debug)]
    struct TestTuple<T: Trait>(T::Type);

    struct Test;
    impl Trait for Test {
        type Type = String;
    }

    let test: TestTuple<Test> = TestTuple(String::from("hi"));
    let cloned = test.clone();

    dbg!(test);
    dbg!(cloned);
}

#[test]
fn test_tuple() {
    #[derive_where(T, S; Clone)]
    #[derive(Debug)]
    struct TestTuple<T, S>(T, S);

    let test = TestTuple(1, String::from("Test"));
    let cloned = test.clone();

    dbg!(test);
    dbg!(cloned);
}

#[test]
fn test_struct() {
    // test macro hygiene
    #[allow(non_upper_case_globals)]
    const a: usize = 0;

    #[derive_where(T, S; Clone)]
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
    #[derive_where(T, S; Clone)]
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
