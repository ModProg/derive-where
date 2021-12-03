use std::marker::PhantomData;

use derive_where::DeriveWhere;

#[derive(DeriveWhere)]
#[derive_where(skip_inner, Clone)]
struct SkipInnerWithTrait<T>(PhantomData<T>);

#[derive(DeriveWhere)]
#[derive_where(Debug = invalid; T)]
struct WrongOptionSyntax<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(,)]
struct OnlyComma<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(,Clone)]
struct StartWithComma<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Clone,,)]
struct DuplicateCommaAtEnd<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(T)]
struct GenericInsteadTrait<T, U>(T, PhantomData<U>);

fn main() {}
