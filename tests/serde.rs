mod util;

use std::marker::PhantomData;

use derive_where::derive_where;
#[cfg(not(feature = "serde"))]
use serde_::{Deserialize, Serialize};
use serde_test::Token;
use util::Wrapper;

#[test]
fn basic() {
	#[derive_where(Debug, PartialEq; T)]
	#[cfg_attr(
		feature = "serde",
		derive_where(Deserialize, Serialize; T)
	)]
	#[cfg_attr(not(feature = "serde"), derive(Deserialize, Serialize))]
	#[cfg_attr(
		not(feature = "serde"),
		serde(bound(deserialize = "T: Deserialize<'de>", serialize = "T: Serialize"))
	)]
	#[serde(crate = "serde_")]
	struct Test<T, U>(T, PhantomData<U>);

	let test = Test(42, PhantomData::<i32>);

	serde_test::assert_tokens(
		&test,
		&[
			Token::TupleStruct {
				name: "Test",
				len: 2,
			},
			Token::I32(42),
			Token::UnitStruct {
				name: "PhantomData",
			},
			Token::TupleStructEnd,
		],
	);
}

#[test]
fn attribute() {
	#[derive_where(Debug, PartialEq)]
	#[cfg_attr(feature = "serde", derive_where(Deserialize, Serialize))]
	#[cfg_attr(not(feature = "serde"), derive(Deserialize, Serialize))]
	#[cfg_attr(not(feature = "serde"), serde(bound = ""))]
	#[serde(crate = "serde_")]
	#[serde(transparent)]
	struct Test<T>(Wrapper<T>);

	let test = Test(42.into());

	serde_test::assert_tokens(
		&test,
		&[
			Token::Struct {
				name: "Wrapper",
				len: 2,
			},
			Token::Str("data"),
			Token::I32(42),
			Token::Str("hack"),
			Token::UnitStruct {
				name: "PhantomData",
			},
			Token::StructEnd,
		],
	);
}
