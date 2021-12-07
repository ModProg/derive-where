use std::marker::PhantomData;

use derive_where::DeriveWhere;

#[derive(DeriveWhere)]
#[derive_where(Debug)]
struct DefaultOnStruct<T>(#[derive_where(default)] PhantomData<T>);

#[derive(DeriveWhere)]
#[derive_where(Clone)]
enum DefaultWithoutTrait<T> {
	#[derive_where(default)]
	A(PhantomData<T>),
}

#[derive(DeriveWhere)]
#[derive_where(Default)]
enum MissingDefault<T> {
	A(PhantomData<T>),
}

#[derive(DeriveWhere)]
#[derive_where(Default)]
enum DuplicateDefaultSeparate<T> {
	#[derive_where(default)]
	A(PhantomData<T>),
	#[derive_where(default)]
	B(PhantomData<T>),
}

#[derive(DeriveWhere)]
#[derive_where(Default)]
enum DuplicateDefaultSame<T> {
	#[derive_where(default, default)]
	A(PhantomData<T>),
}

#[derive(DeriveWhere)]
#[derive_where(Default)]
enum DuplicateDefaultSameSeparate<T> {
	#[derive_where(default)]
	#[derive_where(default)]
	A(PhantomData<T>),
}

fn main() {}
