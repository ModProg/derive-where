use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where(crate = struct Test)]
struct InvalidPath<T>(PhantomData<T>);

#[derive_where(skip_inner, Clone)]
struct SkipInnerWithTrait<T>(PhantomData<T>);

#[derive_where(,)]
struct OnlyComma<T>(PhantomData<T>);

#[derive_where(,Clone)]
struct StartWithComma<T>(PhantomData<T>);

#[derive_where(Clone,,)]
struct DuplicateCommaAtEnd<T>(PhantomData<T>);

#[derive_where("Clone")]
struct InvalidTrait<T>(PhantomData<T>);

#[derive_where(T)]
struct GenericInsteadTrait<T, U>(T, PhantomData<U>);

fn main() {}
