use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where(Clone; T)]
#[derive_where(skip_inner(Clone))]
struct UnsupportedTrait<T>(PhantomData<T>);

fn main() {}
