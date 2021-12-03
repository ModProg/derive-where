use core::marker::PhantomData;

use derive_where::DeriveWhere;

#[derive(DeriveWhere)]
#[derive_where(Debug)]
union UnsupportedTrait<T> {
	a: u8,
	b: PhantomData<T>,
}

fn main() {}
