#![allow(clippy::clone_on_copy)]

mod util;

use derive_where::derive_where;

use self::util::{AssertClone, AssertCopy, Wrapper};

#[test]
fn single() {
	#[derive_where(Clone, Copy)]
	union Test<T> {
		a: Wrapper<T>,
	}

	let test_1 = Test { a: 42.into() };

	let _ = AssertClone(&test_1);
	let _ = AssertCopy(&test_1);

	let test_clone = test_1.clone();
	assert_eq!(unsafe { test_clone.a }, 42);

	let test_copy = test_1;
	assert_eq!(unsafe { test_copy.a }, 42);
}

#[test]
fn multiple() {
	#[derive_where(Clone, Copy)]
	union Test<T> {
		a: Wrapper<T>,
		b: Wrapper<T>,
		c: Wrapper<T>,
	}

	let test_1 = Test { a: 42.into() };
	let test_2 = Test { b: 43.into() };
	let test_3 = Test { c: 44.into() };

	let _ = AssertClone(&test_1);
	let _ = AssertCopy(&test_1);

	let test_clone = test_1.clone();
	assert_eq!(unsafe { test_clone.a }, 42);
	let test_clone = test_2.clone();
	assert_eq!(unsafe { test_clone.b }, 43);
	let test_clone = test_3.clone();
	assert_eq!(unsafe { test_clone.c }, 44);

	let test_copy = test_1;
	assert_eq!(unsafe { test_copy.a }, 42);
	let test_copy = test_2;
	assert_eq!(unsafe { test_copy.b }, 43);
	let test_copy = test_3;
	assert_eq!(unsafe { test_copy.c }, 44);
}
