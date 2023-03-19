use derive_where::derive_where;

use crate::util::{AssertDebug, Wrapper};

#[test]
fn all() {
	#[derive_where(Debug)]
	#[derive_where(skip_inner)]
	struct Test<T> {
		a: Wrapper<T>,
	}

	let test_1 = Test { a: 42.into() };

	let _ = AssertDebug(&test_1);

	assert_eq!(format!("{:?}", test_1), "Test { .. }");
}

#[test]
fn partial() {
	#[derive_where(Debug)]
	struct Test<T> {
		#[derive_where(skip)]
		a: Wrapper<T>,
		b: Wrapper<T>,
	}

	let test_1 = Test {
		a: 42.into(),
		b: 42.into(),
	};

	let _ = AssertDebug(&test_1);

	assert_eq!(format!("{:?}", test_1), "Test { b: 42, .. }");
}
