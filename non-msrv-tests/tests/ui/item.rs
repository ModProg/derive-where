use std::marker::PhantomData;

use derive_where::DeriveWhere;

#[derive(DeriveWhere)]
struct NoAttribute<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where]
struct NoOption<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where()]
struct EmptyAttribute<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Clone; T;)]
struct SemiColonAtTheEnd<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Clone; T,,)]
struct DoubleColonAtTheEnd<T, U>(T, PhantomData<U>);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Clone; where)]
struct InvalidGeneric<T, U>(T, PhantomData<U>);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Clone Debug; T)]
struct MissingCommaBetweenTraits<T, U>(T, PhantomData<U>);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Clone; T U)]
struct MissingCommaBetweenGenerics<T, U, V>(T, PhantomData<(U, V)>);

#[derive(DeriveWhere)]
#[derive_where("Clone")]
struct InvalidTrait<T, U>(T, PhantomData<U>);

fn main() {}
