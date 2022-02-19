use std::marker::PhantomData;

use derive_where::derive_where;

#[test]
fn cfg() {
	#[derive_where(Clone)]
	struct Test<T> {
		a: PhantomData<T>,
		#[cfg(invalid)]
		b: u8,
	}
}
