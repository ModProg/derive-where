use std::{
	fmt,
	fmt::{Debug, Formatter},
	marker::PhantomData,
	ptr,
};

#[cfg(feature = "zeroize")]
use zeroize_::Zeroize;
#[cfg(feature = "zeroize-on-drop")]
use zeroize_::ZeroizeOnDrop;

pub struct Wrapper<T = ()> {
	data: i32,
	hack: PhantomData<T>,
}

impl<T> Debug for Wrapper<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.data)
	}
}

impl From<i32> for Wrapper<()> {
	fn from(data: i32) -> Self {
		Self {
			data,
			hack: PhantomData,
		}
	}
}

impl<T> PartialEq<i32> for Wrapper<T> {
	fn eq(&self, other: &i32) -> bool {
		&self.data == other
	}
}

#[cfg(feature = "zeroize")]
impl<T> Zeroize for Wrapper<T> {
	fn zeroize(&mut self) {
		self.data.zeroize();
	}
}

#[cfg(feature = "zeroize")]
pub struct AssertZeroize<'a, T: Zeroize>(pub &'a T);

#[cfg(feature = "zeroize-on-drop")]
pub struct AssertZeroizeOnDrop<'a, T: ZeroizeOnDrop>(pub &'a T);

pub fn test_drop<T>(mut value: T, fun: impl FnOnce(T)) {
	unsafe { ptr::drop_in_place(&mut value) };
	fun(value);
}
