extern crate zeroize_ as zeroize;

use std::marker::PhantomData;

use derive_where::DeriveWhere;

#[derive(DeriveWhere)]
#[derive_where(Zeroize(test))]
struct InvalidOption<T>(PhantomData<T>);

#[derive(DeriveWhere)]
#[derive_where(Zeroize(test = "test"))]
struct WrongOptionSyntax1<T>(PhantomData<T>);

#[derive(DeriveWhere)]
#[derive_where(Zeroize("option"))]
struct WrongOptionSyntax2<T>(PhantomData<T>);

#[derive(DeriveWhere)]
#[derive_where(Zeroize(crate(zeroize_)))]
struct WrongCrateSyntax<T>(PhantomData<T>);

#[derive(DeriveWhere)]
#[derive_where(Zeroize(crate = "struct Test"))]
struct InvalidCrate<T>(PhantomData<T>);

#[derive(DeriveWhere)]
#[derive_where(Zeroize(drop, drop))]
struct DuplicateDrop<T>(PhantomData<T>);

#[derive(DeriveWhere)]
#[derive_where(Zeroize(crate = "zeroize_", crate = "zeroize_"))]
struct DuplicateCrate<T>(PhantomData<T>);

#[derive(DeriveWhere)]
#[derive_where(Zeroize(drop, drop, crate = "zeroize_"))]
struct DuplicateDropWithCrate<T>(PhantomData<T>);

#[derive(DeriveWhere)]
#[derive_where(Zeroize(drop, crate = "zeroize_", crate = "zeroize_"))]
struct DropWithDuplicateCrate<T>(PhantomData<T>);

#[derive(DeriveWhere)]
#[derive_where(Zeroize(crate = "zeroize_", crate = "zeroize_", drop))]
struct DuplicateCrateWithDrop<T>(PhantomData<T>);

#[derive(DeriveWhere)]
#[derive_where(Zeroize(crate = "zeroize_", drop, drop))]
struct CrateWithDuplicateDrop<T>(PhantomData<T>);

fn main() {}
