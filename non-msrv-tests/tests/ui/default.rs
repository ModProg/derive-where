use core::marker::PhantomData;

use derive_where::DeriveWhere;

#[derive(DeriveWhere)]
#[derive_where(Debug; T)]
struct DefaultOnStruct<T, U>(#[derive_where(default)] T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Clone; T)]
enum DefaultWithoutTrait<T, U> {
	#[derive_where(default)]
	A(T),
	B(PhantomData<U>),
}

#[derive(DeriveWhere)]
#[derive_where(Default; T)]
enum MissingDefault<T, U> {
	A(T),
	B(PhantomData<U>),
}

#[derive(DeriveWhere)]
#[derive_where(Default; T)]
enum DuplicateDefaultSeparate<T, U> {
	#[derive_where(default)]
	A(T),
	#[derive_where(default)]
	B(PhantomData<U>),
}

#[derive(DeriveWhere)]
#[derive_where(Default; T)]
enum DuplicateDefaultSame<T, U> {
	#[derive_where(default, default)]
	A(T),
	B(PhantomData<U>),
}

#[derive(DeriveWhere)]
#[derive_where(Default; T)]
enum DuplicateDefaultSameSeparate<T, U> {
	#[derive_where(default)]
	#[derive_where(default)]
	A(T),
	B(PhantomData<U>),
}

fn main() {}
