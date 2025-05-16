extern crate serde_ as serde;

use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where(Clone)]
#[serde(crate = "serde_")]
struct InvalidSerde<T>(PhantomData<T>);

fn main() {}
