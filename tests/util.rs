use core::{
	cmp::Ordering,
	fmt,
	fmt::{Debug, Formatter},
	hash::{Hash, Hasher},
	marker::PhantomData,
};

pub struct Wrapper<T = ()> {
	data: i32,
	hack: PhantomData<T>,
}

impl<T> Clone for Wrapper<T> {
	fn clone(&self) -> Self {
		Self {
			data: self.data,
			hack: self.hack,
		}
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
		self.data.partial_cmp(&other.data)
	}
}
