extern crate zeroize_ as zeroize;

use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where(Zeroize(drop))]
struct DeprecatedDrop<T>(PhantomData<T>);

fn main() {}
