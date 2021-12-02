#![no_std]

use core::marker::PhantomData;

// TODO: ensure more things then just `Clone`.

#[derive(derive_where::DeriveWhere)]
#[derive_where(Clone)]
pub struct Test<T>(PhantomData<T>);
