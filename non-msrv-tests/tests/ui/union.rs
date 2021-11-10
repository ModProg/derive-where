#[allow(unused_imports)]
use core::marker::PhantomData;

#[derive_where::derive_where(Debug)]
union Test<T> {
    field: PhantomData<T>,
}

fn main() {}
