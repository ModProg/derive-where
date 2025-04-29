use std::{
	cmp::Ordering,
	collections::hash_map::DefaultHasher,
	fmt,
	fmt::{Debug, Formatter},
	hash::{Hash, Hasher},
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

impl<T> Clone for Wrapper<T> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<T> Copy for Wrapper<T> {}

impl<T> Debug for Wrapper<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.data)
	}
}

impl<T> Default for Wrapper<T> {
	fn default() -> Self {
		Self {
			data: i32::default(),
			hack: PhantomData,
		}
	}
}

impl<T> Eq for Wrapper<T> {}

impl From<i32> for Wrapper<()> {
	fn from(data: i32) -> Self {
		Self {
			data,
			hack: PhantomData,
		}
	}
}

impl<T> Hash for Wrapper<T> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.data.hash(state);
	}
}

impl<T> Ord for Wrapper<T> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.data.cmp(&other.data)
	}
}

impl<T> PartialEq for Wrapper<T> {
	fn eq(&self, other: &Self) -> bool {
		self.data == other.data
	}
}

impl<T> PartialEq<i32> for Wrapper<T> {
	fn eq(&self, other: &i32) -> bool {
		&self.data == other
	}
}

impl<T> PartialOrd for Wrapper<T> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

#[cfg(feature = "zeroize")]
impl<T> Zeroize for Wrapper<T> {
	fn zeroize(&mut self) {
		self.data.zeroize();
	}
}

#[allow(dead_code)]
pub struct AssertClone<'a, T: Clone>(pub &'a T);

#[allow(dead_code)]
pub struct AssertCopy<'a, T: Copy>(pub &'a T);

#[allow(dead_code)]
pub struct AssertDebug<'a, T: Debug>(pub &'a T);

#[allow(dead_code)]
pub struct AssertEq<'a, T: Eq>(pub &'a T);

#[allow(dead_code)]
pub struct AssertHash<'a, T: Hash>(pub &'a T);

#[allow(dead_code)]
pub struct AssertOrd<'a, T: Ord>(pub &'a T);

#[allow(dead_code)]
pub struct AssertPartialEq<'a, T: PartialEq>(pub &'a T);

#[allow(dead_code)]
pub struct AssertPartialOrd<'a, T: PartialOrd>(pub &'a T);

#[cfg(feature = "zeroize")]
#[allow(dead_code)]
pub struct AssertZeroize<'a, T: Zeroize>(pub &'a T);

#[cfg(feature = "zeroize-on-drop")]
#[allow(dead_code)]
pub struct AssertZeroizeOnDrop<'a, T: ZeroizeOnDrop>(pub &'a T);

#[allow(dead_code)]
pub struct NonTrait<T = ()> {
	data: i32,
	hack: PhantomData<T>,
}

#[allow(dead_code)]
impl<T> NonTrait<T> {
	pub fn data(&self) -> i32 {
		self.data
	}
}

impl<T> Default for NonTrait<T> {
	fn default() -> Self {
		Self {
			data: i32::default(),
			hack: PhantomData,
		}
	}
}

impl From<i32> for NonTrait<()> {
	fn from(data: i32) -> Self {
		Self {
			data,
			hack: PhantomData,
		}
	}
}

#[allow(dead_code)]
pub fn hash_eq<T: Hash>(test_1: T, test_2: T) {
	let mut hasher = DefaultHasher::new();
	test_1.hash(&mut hasher);
	let hash_1 = hasher.finish();
	let mut hasher = DefaultHasher::new();
	test_2.hash(&mut hasher);
	let hash_2 = hasher.finish();
	assert_eq!(hash_1, hash_2);
}

#[allow(dead_code)]
pub fn hash_ne<T: Hash>(test_1: T, test_2: T) {
	let mut hasher = DefaultHasher::new();
	test_1.hash(&mut hasher);
	let hash_1 = hasher.finish();
	let mut hasher = DefaultHasher::new();
	test_2.hash(&mut hasher);
	let hash_2 = hasher.finish();
	assert_ne!(hash_1, hash_2);
}

#[allow(dead_code)]
pub fn test_drop<T>(mut value: T, fun: impl FnOnce(T)) {
	unsafe { ptr::drop_in_place(&mut value) };
	fun(value);
}
