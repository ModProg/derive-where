use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where(Default)]
struct StructNoOption<T>(#[derive_where] PhantomData<T>);

#[derive_where(Default)]
struct StructWrongSyntax<T>(#[derive_where = "default"] PhantomData<T>);

#[derive_where(Default)]
enum EnumNoOption<T> {
	#[derive_where]
	A(PhantomData<T>),
}

#[derive_where(Default)]
enum EnumWrongSyntax<T> {
	#[derive_where = "default"]
	A(PhantomData<T>),
}

#[derive_where(Clone)]
struct StructInvalidOption<T>(#[derive_where(option)] PhantomData<T>);

#[derive_where(Clone)]
enum EnumInvalidOption<T> {
	#[derive_where(option)]
	A(PhantomData<T>),
}

fn main() {}
