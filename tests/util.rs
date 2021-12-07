use std::{
	cmp::Ordering,
	collections::hash_map::DefaultHasher,
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

pub struct AssertClone<T: Clone>(pub T);

pub struct AssertCopy<T: Copy>(pub T);

#[allow(dead_code)]
pub struct AssertDebug<T: Debug>(pub T);

#[allow(dead_code)]
pub struct AssertEq<T: Eq>(pub T);

#[allow(dead_code)]
pub struct AssertHash<T: Hash>(pub T);

#[allow(dead_code)]
pub struct AssertOrd<T: Ord>(pub T);

#[allow(dead_code)]
pub struct AssertPartialEq<T: PartialEq>(pub T);

#[allow(dead_code)]
pub struct AssertPartialOrd<T: PartialOrd>(pub T);

// Copied from std. Changed `pat_param` to `pat` to support MSRV.
#[allow(unused_macros)]
macro_rules! matches {
    ($expression:expr, $(|)? $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => {
        match $expression {
            $( $pattern )|+ $( if $guard )? => true,
            _ => false
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
