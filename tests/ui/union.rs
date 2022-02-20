use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where(Debug)]
union UnsupportedTrait<T> {
	a: Option<PhantomData<T>>,
}

#[derive_where(Clone)]
union MissingCopy<T> {
	a: Option<PhantomData<T>>,
}

fn main() {}
