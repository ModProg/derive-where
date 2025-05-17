use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where(Clone)]
#[serde(crate = "serde_")]
struct InvalidAttribute<T>(PhantomData<T>);

fn main() {}
