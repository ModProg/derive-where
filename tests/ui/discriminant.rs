use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where(PartialEq, PartialOrd)]
#[repr(A)]
enum UnknownRepr<T> {
	A(PhantomData<T>),
}

#[derive_where(PartialEq, PartialOrd)]
enum MissingIntegerRepr<T> {
	A(PhantomData<T>) = 0,
}

#[derive_where(PartialEq, PartialOrd)]
#[repr(C)]
enum MissingIntegerReprC<T> {
	A(PhantomData<T>) = 0,
}

fn main() {}
