#![allow(clippy::clone_on_copy)]

use std::collections::hash_map::DefaultHasher;
use core::hash::{Hash, Hasher};
use core::fmt::Debug;

use derive_where::derive_where;

struct AssertClone<T: Clone>(T);
struct AssertCopy<T: Copy>(T);
struct AssertDebug<T: Debug>(T);
struct AssertEq<T: Eq>(T);
struct AssertHash<T: Hash>(T);
struct AssertOrd<T: Ord>(T);
struct AssertPartialEq<T: PartialEq>(T);
struct AssertPartialOrd<T: PartialOrd>(T);

#[test]
fn struct_single() {
	#[derive_where(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd; T)]
	struct Test<T> {
		a: T,
	}

	let test_1 = Test { a: 42 };
	let test_2 = Test { a: 42 };

	let _ = AssertClone(test_1);
	let _ = AssertCopy(test_1);
	let _ = AssertDebug(test_1);
	let _ = AssertEq(test_1);
	let _ = AssertHash(test_1);
	let _ = AssertOrd(test_1);
	let _ = AssertPartialEq(test_1);
	let _ = AssertPartialOrd(test_1);

	let test_clone = test_1.clone();
	assert_eq!(test_clone.a, 42);

	let test_copy = test_1;
	assert_eq!(test_copy.a, 42);

	assert_eq!(format!("{:?}", test_1), "Test { a: 42 }");

	let mut hasher = DefaultHasher::new();
	test_1.hash(&mut hasher);
	let hash_1 = hasher.finish();
	let mut hasher = DefaultHasher::new();
	test_2.hash(&mut hasher);
	let hash_2 = hasher.finish();
	assert_eq!(hash_1, hash_2);

	assert!(test_1 == test_2);
	assert!(test_1 != Test { a: 43 });

	assert!(test_1 > Test { a: 41 });
	assert!(test_1 < Test { a: 43 });
}

#[test]
fn struct_multiple() {
	#[derive_where(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd; T)]
	struct Test<T> {
		a: T,
		b: T,
		c: T,
	}

	let test_1 = Test { a: 42, b: 43, c: 44 };
	let test_2 = Test { a: 42, b: 43, c: 44 };

	let _ = AssertClone(test_1);
	let _ = AssertCopy(test_1);
	let _ = AssertDebug(test_1);
	let _ = AssertEq(test_1);
	let _ = AssertHash(test_1);
	let _ = AssertOrd(test_1);
	let _ = AssertPartialEq(test_1);
	let _ = AssertPartialOrd(test_1);

	let test_clone = test_1.clone();
	assert_eq!(test_clone.a, 42);
	assert_eq!(test_clone.b, 43);
	assert_eq!(test_clone.c, 44);

	let test_copy = test_1;
	assert_eq!(test_copy.a, 42);
	assert_eq!(test_copy.b, 43);
	assert_eq!(test_copy.c, 44);

	assert_eq!(format!("{:?}", test_1), "Test { a: 42, b: 43, c: 44 }");

	let mut hasher = DefaultHasher::new();
	test_1.hash(&mut hasher);
	let hash_1 = hasher.finish();
	let mut hasher = DefaultHasher::new();
	test_2.hash(&mut hasher);
	let hash_2 = hasher.finish();
	assert_eq!(hash_1, hash_2);

	assert!(test_1 == test_2);
	assert!(test_1 != Test { a: 43, b: 43, c: 44 });
	assert!(test_1 != Test { a: 42, b: 44, c: 44 });
	assert!(test_1 != Test { a: 42, b: 43, c: 45 });
	assert!(test_1 != Test { a: 45, b: 45, c: 45 });

	assert!(test_1 > Test { a: 41, b: 43, c: 44 });
	assert!(test_1 > Test { a: 42, b: 42, c: 44 });
	assert!(test_1 > Test { a: 42, b: 43, c: 43 });
	assert!(test_1 < Test { a: 43, b: 43, c: 44 });
	assert!(test_1 < Test { a: 42, b: 44, c: 44 });
	assert!(test_1 < Test { a: 42, b: 43, c: 45 });
}
