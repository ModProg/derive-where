use std::marker::PhantomData;

use derive_where::DeriveWhere;

#[derive(DeriveWhere)]
#[derive_where(Debug)]
union UnsupportedTrait<T> {
	a: u8,
	b: PhantomData<T>,
}

#[derive(DeriveWhere)]
#[derive_where(Clone)]
union MissingCopy<T> {
	a: u8,
	b: PhantomData<T>,
}

fn main() {}
