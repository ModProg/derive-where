use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where()]
struct EmptyAttribute<T>(PhantomData<T>);

#[derive_where(Debug = "option")]
struct WrongOptionSyntax<T>(PhantomData<T>);

#[derive_where(Debug())]
struct EmptyOption<T>(PhantomData<T>);

#[derive_where(Debug(option))]
struct UnsupportedOption<T>(PhantomData<T>);

fn main() {}
