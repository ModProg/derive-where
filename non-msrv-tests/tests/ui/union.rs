use std::marker::PhantomData;

use derive_where::DeriveWhere;

#[derive(DeriveWhere)]
#[derive_where(Debug)]
union UnsupportedTrait<T> {
	a: Option<PhantomData<T>>,
}

#[derive(DeriveWhere)]
#[derive_where(Clone)]
union MissingCopy<T> {
	a: Option<PhantomData<T>>,
}

fn main() {}
