use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where(Deserialize)]
#[serde(bound = "")]
struct ConflictingBound<T>(PhantomData<T>);

fn main() {}
