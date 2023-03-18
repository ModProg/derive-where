extern crate zeroize_ as zeroize;

use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where(Zeroize(test))]
struct InvalidOption<T>(PhantomData<T>);

#[derive_where(Zeroize(test = "test"))]
struct WrongOptionSyntax1<T>(PhantomData<T>);

#[derive_where(Zeroize("option"))]
struct WrongOptionSyntax2<T>(PhantomData<T>);

#[derive_where(Zeroize(crate(zeroize_)))]
struct WrongCrateSyntax<T>(PhantomData<T>);

#[derive_where(Zeroize(crate = struct Test))]
struct InvalidPath<T>(PhantomData<T>);

#[derive_where(Zeroize(crate = "struct Test"))]
struct InvalidPathDeprecated<T>(PhantomData<T>);

#[derive_where(Zeroize(crate = zeroize_, crate = zeroize_))]
struct DuplicateCrate<T>(PhantomData<T>);

#[derive_where(Zeroize(crate = ::zeroize))]
struct DefaultCrate<T>(PhantomData<T>);

fn main() {}
