#![no_std]

#[cfg(feature = "zeroize")]
extern crate zeroize_ as zeroize;

use core::marker::PhantomData;

#[derive(derive_where::DeriveWhere)]
#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "zeroize", derive_where(Zeroize))]
pub struct Test<T>(#[cfg_attr(feature = "zeroize", derive_where(skip(Zeroize)))] PhantomData<T>);
