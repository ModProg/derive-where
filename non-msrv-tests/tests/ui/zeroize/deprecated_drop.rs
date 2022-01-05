extern crate zeroize_ as zeroize;

use std::marker::PhantomData;

use derive_where::DeriveWhere;

#[derive(DeriveWhere)]
#[derive_where(Zeroize(drop))]
struct DeprecatedDrop<T>(PhantomData<T>);

fn main() {}
