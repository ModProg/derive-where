use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where = "invalid"]
struct WrongAttributeSyntax<T>(PhantomData<T>);

fn main() {}
