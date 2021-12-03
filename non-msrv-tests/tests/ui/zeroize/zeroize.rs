extern crate zeroize_ as zeroize;

use core::marker::PhantomData;

use derive_where::DeriveWhere;

#[derive(DeriveWhere)]
#[derive_where(Zeroize(test); T)]
struct InvalidOption<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Zeroize(test = "test"); T)]
struct WrongOptionSyntax1<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Zeroize("option"); T)]
struct WrongOptionSyntax2<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Zeroize(crate(zeroize_)); T)]
struct WrongCrateSyntax<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Zeroize(crate = "struct Test"); T)]
struct InvalidCrate<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Zeroize(drop, drop); T)]
struct DuplicateDrop<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Zeroize(crate = "zeroize_", crate = "zeroize_"); T)]
struct DuplicateCrate<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Zeroize(drop, drop, crate = "zeroize_"); T)]
struct DuplicateDropWithCrate<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Zeroize(drop, crate = "zeroize_", crate = "zeroize_"); T)]
struct DropWithDuplicateCrate<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Zeroize(crate = "zeroize_", crate = "zeroize_", drop); T)]
struct DuplicateCrateWithDrop<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Zeroize(crate = "zeroize_", drop, drop); T)]
struct CrateWithDuplicateDrop<T, U>(T, PhantomData<U>);

fn main() {}
