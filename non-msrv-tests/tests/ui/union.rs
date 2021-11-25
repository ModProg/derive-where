#[allow(unused_imports)]
use core::marker::PhantomData;

#[derive(derive_where::DeriveWhere)]
#[derive_where(Debug)]
union Test<T> {
    field: PhantomData<T>,
}

fn main() {}
