use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where(Copy; T)]
struct UnsupportedTrait<T>(#[derive_where(skip(Copy))] PhantomData<T>);

fn main() {}
