extern crate zeroize_ as zeroize;

use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where(ZeroizeOnDrop(no_drop))]
struct InvalidOption<T>(PhantomData<T>);

fn main() {}
