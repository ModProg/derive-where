use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where(Deserialize)]
struct UnsupportedDeserialize<T>(PhantomData<T>);

#[derive_where(Serialize)]
struct UnsupportedSerialize<T>(PhantomData<T>);

fn main() {}
