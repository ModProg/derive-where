use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where(Clone)]
#[serde(crate = "serde_")]
struct ValidSerde<T>(PhantomData<T>);

#[derive_where(Deserialize)]
#[serde(bound = "")]
struct ConflictingBound<T>(PhantomData<T>);

fn main() {}
