#![cfg(feature = "serde")]

use std::marker::PhantomData;

use derive_where::derive_where;

#[test]
fn test() {
	#[derive_where(Deserialize, Serialize; T)]
	#[serde(crate = "serde_")]
	struct Test<T, U>(T, PhantomData<U>);
}
