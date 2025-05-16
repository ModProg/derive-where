#![no_std]

use core::marker::PhantomData;

use derive_where_::derive_where;

#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
	feature = "serde",
	derive_where(Deserialize(crate = serde_), Serialize(crate = serde_))
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_"))]
#[cfg_attr(feature = "zeroize", derive_where(Zeroize(crate = zeroize_)))]
#[derive_where(crate = derive_where_)]
pub struct Test<T>(PhantomData<T>);

#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
	feature = "serde",
	derive_where(Deserialize(crate = "serde_"), Serialize(crate = "serde_"))
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_"))]
#[cfg_attr(feature = "zeroize", derive_where(Zeroize(crate = "zeroize_")))]
#[derive_where(crate = "derive_where_")]
pub struct TestDeprecated<T>(PhantomData<T>);
