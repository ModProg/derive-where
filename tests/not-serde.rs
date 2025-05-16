#![cfg(not(feature = "serde"))]

use std::marker::PhantomData;

use derive_where::derive_where;
use serde_::{Deserialize, Serialize};
use serde_test::Token;

#[test]
fn test() {
	#[derive(Deserialize, Serialize)]
	#[derive_where(Debug, PartialEq)]
	#[serde(crate = "serde_")]
	struct Test<T>(PhantomData<T>);

	let test = Test::<()>(PhantomData);

	serde_test::assert_tokens(
		&test,
		&[
			Token::NewtypeStruct { name: "Test" },
			Token::UnitStruct {
				name: "PhantomData",
			},
		],
	);
}
