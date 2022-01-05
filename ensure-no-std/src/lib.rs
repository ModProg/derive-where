#![no_std]

#[cfg(feature = "zeroize")]
extern crate zeroize_ as zeroize;

use core::marker::PhantomData;

use derive_where::DeriveWhere;

#[derive(DeriveWhere)]
#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "zeroize", derive_where(Zeroize))]
pub struct Test<T>(PhantomData<T>);
