#![cfg(feature = "serde")]

use derive_where::derive_where;

#[test]
fn test() {
	#[derive_where(crate = derive_where)]
	#[derive_where(Deserialize)]
	struct Test<T>(T);
}
