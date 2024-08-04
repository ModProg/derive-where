#![allow(unused)]
mod util;

use derive_where::derive_where;

use self::util::Wrapper;

#[test]
fn struct_() {
	#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
	struct Test<T> {
		r#as: Wrapper<T>,
		r#break: Wrapper<T>,
		r#const: Wrapper<T>,
	}
}

#[test]
fn enum_() {
	#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
	enum Test<T> {
		#[derive_where(default)]
		A {
			r#as: Wrapper<T>,
			r#break: Wrapper<T>,
			r#const: Wrapper<T>,
		},
	}
}
