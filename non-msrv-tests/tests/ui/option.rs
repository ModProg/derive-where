use core::marker::PhantomData;

use derive_where::DeriveWhere;

#[derive(DeriveWhere)]
#[derive_where(Default; T)]
struct StructNoOption<T, U>(#[derive_where] T, PhantomData<U>);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Default; T)]
struct StructWrongSyntax<T, U>(#[derive_where = "default"] T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Default; T)]
enum EnumNoOption<T, U> {
	#[derive_where]
	A(T),
	B(PhantomData<U>),
}

#[derive(DeriveWhere)]
#[derive_where(Default; T)]
enum EnumWrongSyntax<T, U> {
	#[derive_where = "default"]
	A(T),
	B(PhantomData<U>),
}

#[derive(DeriveWhere)]
#[derive_where(Clone; T)]
struct StructInvalidOption<T, U>(#[derive_where(option)] T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Clone; T)]
enum EnumInvalidOption<T, U> {
	#[derive_where(option)]
	A(T),
	B(PhantomData<U>),
}

fn main() {}
