use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where(Zeroize)]
struct Unsupported<T>(PhantomData<T>);

fn main() {}
