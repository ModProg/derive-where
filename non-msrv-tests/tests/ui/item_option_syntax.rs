use std::marker::PhantomData;

use derive_where::DeriveWhere;

#[derive(DeriveWhere)]
// Rust itself already fails to parse this and will provide a separate error message.
#[derive_where = invalid]
struct InvalidAttribute<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where = "invalid"]
struct WrongAttributeSyntax<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where()]
struct EmptyAttribute<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Debug = "option"; T)]
struct WrongOptionSyntax<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Debug(); T)]
struct EmptyOption<T, U>(T, PhantomData<U>);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Debug(option); T)]
struct UnsupportedOption<T, U>(T, PhantomData<U>);

fn main() {}
