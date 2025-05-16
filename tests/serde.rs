#![cfg(feature = "serde")]

use derive_where::derive_where;

#[test]
fn test() {
	#[derive_where(Deserialize, Serialize)]
	#[serde(crate = "serde_")]
	struct Test<T>(T);
}
