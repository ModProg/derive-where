use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where(Skip; T)]
struct UnsupportedTrait<T>(#[derive_where(skip(Skip))] PhantomData<T>);

fn main() {}
