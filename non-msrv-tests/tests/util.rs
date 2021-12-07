use std::{
	fmt,
	fmt::{Debug, Formatter},
	marker::PhantomData,
	ptr,
};

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
impl<T> zeroize_::Zeroize for Wrapper<T> {
	fn zeroize(&mut self) {
		self.data.zeroize();
	}
}

pub fn test_drop<T>(value: T, fun: impl FnOnce(&T)) {
	let mut test_holder = vec![value];
	let ptr = &mut test_holder[0] as *mut T;

	let test = unsafe {
		test_holder.set_len(0);
		ptr::drop_in_place(ptr);
		&*ptr
	};

	assert_eq!(test_holder.capacity(), 1);
	fun(test);
}
