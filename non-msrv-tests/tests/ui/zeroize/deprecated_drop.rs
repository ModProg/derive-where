use std::marker::PhantomData;

extern crate zeroize_ as zeroize;

use derive_where::derive_where;

#[derive_where(Zeroize(drop))]
struct DeprecatedDrop<T>(PhantomData<T>);

fn main() {}
