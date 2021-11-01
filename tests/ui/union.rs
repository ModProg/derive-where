#[allow(unused_imports)]
use core::marker::PhantomData;

#[derive_restricted::derive_where(Clone)]
union Test<T> {
    field: PhantomData<T>,
}

fn main() {}
