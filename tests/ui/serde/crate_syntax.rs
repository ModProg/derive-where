use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where(Deserialize)]
#[serde(crate)]
struct WrongCrateSyntax1<T>(PhantomData<T>);

#[derive_where(Deserialize)]
#[serde(crate())]
struct WrongCrateSyntax2<T>(PhantomData<T>);

#[derive_where(Deserialize)]
#[serde(crate = ())]
struct NotString<T>(PhantomData<T>);

#[derive_where(Deserialize)]
#[serde(crate = "1")]
struct NotPath<T>(PhantomData<T>);

#[derive_where(Deserialize)]
#[serde(crate = "serde_")]
#[serde(crate = "serde_")]
struct Duplicate<T>(PhantomData<T>);

fn main() {}
