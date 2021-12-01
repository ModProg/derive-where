#![no_std]

// TODO: ensure more things then just `Clone`.

#[derive(derive_where::DeriveWhere)]
#[derive_where(Clone; T)]
pub struct Test<T>(T);
