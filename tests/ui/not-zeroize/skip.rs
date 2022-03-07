use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where(Clone; T)]
struct UnsupportedTrait<T>(#[derive_where(skip(Clone))] PhantomData<T>);

fn main() {}
