#![no_std]

#[cfg(feature = "zeroize")]
extern crate zeroize_ as zeroize;

use core::marker::PhantomData;

use derive_where_::derive_where;

#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "zeroize", derive_where(Zeroize))]
#[derive_where(crate = "derive_where_")]
pub struct Test<T>(PhantomData<T>);
