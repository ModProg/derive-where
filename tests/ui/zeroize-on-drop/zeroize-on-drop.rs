extern crate zeroize_ as zeroize;

use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where(ZeroizeOnDrop(no_drop, no_drop))]
struct DuplicateNoDrop<T>(PhantomData<T>);

#[derive_where(ZeroizeOnDrop(no_drop))]
struct NoDropNoZeroizeOnDrop<T>(T);

fn main() {}
