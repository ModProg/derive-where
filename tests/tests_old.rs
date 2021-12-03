// #![allow(dead_code)]
//
// use std::marker::PhantomData;
//
// use derive_where::derive_where;
//
// // test macro hygiene
// trait Clone {}
//
// #[test]
// fn test_path() {
//     trait Trait {
//         type Type;
//     }
//     #[derive_where(Clone, Debug; T::Type)]
//     struct TestTuple<T: Trait>(T::Type);
//
//     struct Test;
//     impl Trait for Test {
//         type Type = String;
//     }
//
//     let test: TestTuple<Test> = TestTuple(String::from("hi"));
//     let cloned = test.clone();
//
//     dbg!(test);
//     dbg!(cloned);
// }
//
// #[test]
// fn test_no_bound() {
//     #[derive_where(Clone, Debug)]
//     struct TestTuple<T>(PhantomData<T>);
//
//     struct NotCloneAble;
//
//     let test = TestTuple::<NotCloneAble>(PhantomData);
//     let cloned = test.clone();
//
//     dbg!(test);
//     dbg!(cloned);
// }
//
// #[test]
// fn test_custom_bound() {
//     trait Trait {}
//
//     #[derive_where(Clone, Debug; T: Trait)]
//     struct TestTuple<T>(PhantomData<T>);
//
//     struct NotCloneAble;
//     impl Trait for NotCloneAble {}
//
//     let test = TestTuple::<NotCloneAble>(PhantomData);
//     let cloned = test.clone();
//
//     dbg!(test);
//     dbg!(cloned);
// }
//
// #[test]
// fn test_tuple() {
//     #[derive_where(Clone; T, S)]
//     #[derive(Debug)]
//     struct TestTuple<T, S>(T, S);
//
//     let test = TestTuple(1, String::from("Test"));
//     let cloned = test.clone();
//
//     dbg!(test);
//     dbg!(cloned);
// }
//
// #[test]
// fn test_struct() {
//     // test macro hygiene
//     #[allow(non_upper_case_globals)]
//     const a: usize = 0;
//
//     #[derive_where(Clone; T, S)]
//     #[derive(Debug)]
//     struct TestStruct<T, S> {
//         a: T,
//         b: S,
//     }
//
//     let test = TestStruct {
//         a: 1,
//         b: String::from("hi"),
//     };
//     let cloned = test.clone();
//
//     dbg!(test);
//     dbg!(cloned);
// }
//
// #[test]
// fn test_enum() {
//     #[derive_where(Clone; T, S)]
//     #[derive(Debug)]
//     #[allow(clippy::enum_variant_names)]
//     enum TestEnum<T, S> {
//         VariantStruct { field: T },
//         VariantTupel(S),
//         Variant,
//     }
//     let test = TestEnum::<u8, &str>::Variant;
//     let cloned = test.clone();
//     dbg!(test);
//     dbg!(cloned);
//
//     let test = TestEnum::<u8, &str>::Variant;
//     let cloned = test.clone();
//     dbg!(test);
//     dbg!(cloned);
//
//     let test = TestEnum::<u8, &str>::VariantTupel("hi");
//     let cloned = test.clone();
//     dbg!(test);
//     dbg!(cloned);
//
//     let test = TestEnum::<u8, &str>::VariantStruct { field: 8 };
//     let cloned = test.clone();
//     dbg!(test);
//     dbg!(cloned);
// }
//
// #[test]
// fn test_all() {
//     #[derive_where(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd;
// T, S)]     #[allow(clippy::enum_variant_names)]
//     enum TestEnum<T, S> {
//         VariantStruct { field: T },
//         VariantTupel(S),
//         Variant,
//     }
//
//     let test1 = TestEnum::<u8, &str>::VariantStruct { field: 42 };
//     let test2 = TestEnum::<u8, &str>::VariantTupel("test");
//     let test3 = TestEnum::<u8, &str>::Variant;
//
//     assert_eq!(test1.clone(), test1);
//     assert_eq!(test2.clone(), test2);
//     assert_eq!(test3.clone(), test3);
// }
