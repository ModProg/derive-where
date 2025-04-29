use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where(Copy; T)]
#[derive_where(skip_inner(Copy))]
struct UnsupportedTrait<T>(PhantomData<T>);

fn main() {}
