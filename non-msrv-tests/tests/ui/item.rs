use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where]
struct NoOption<T>(PhantomData<T>);

#[derive_where()]
struct EmptyAttribute<T>(PhantomData<T>);

#[derive_where(crate(derive_where_))]
struct WrongCrateSyntax<T>(PhantomData<T>);

#[derive_where(crate = "struct Test")]
struct InvalidPath<T>(PhantomData<T>);

#[derive_where(crate = "derive_where_", crate = "derive_where_")]
struct DuplicateCrate1<T>(PhantomData<T>);

#[derive_where(crate = "derive_where_")]
struct OnlyCrate<T>(PhantomData<T>);

#[derive_where(crate = "::derive_where")]
struct DefaultCrate<T>(PhantomData<T>);

#[derive_where(Clone; T;)]
struct SemiColonAtTheEnd<T, U>(T, PhantomData<U>);

#[derive_where(Clone; T,,)]
struct DoubleColonAtTheEnd<T, U>(T, PhantomData<U>);

#[derive_where(Clone; where)]
struct InvalidGeneric<T>(PhantomData<T>);

#[derive_where(Clone Debug)]
struct MissingCommaBetweenTraits<T>(PhantomData<T>);

#[derive_where(Clone; T U)]
struct MissingCommaBetweenGenerics<T, U, V>(T, PhantomData<(U, V)>);

#[derive_where("Clone")]
struct InvalidTrait<T>(PhantomData<T>);

fn main() {}
