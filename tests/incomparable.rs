use derive_where::derive_where;

macro_rules! incomparable {
	($expr:expr) => {
		assert_ne!($expr, $expr);
		assert_eq!($expr.partial_cmp(&$expr), None)
	};
	($a:expr, $b:expr) => {
		assert_ne!($a, $b);
		assert_eq!($a.partial_cmp(&$b), None)
	};
}

#[test]
fn struct_() {
	#[derive(Debug)]
	#[derive_where(PartialEq, PartialOrd)]
	#[derive_where(incomparable)]
	struct Unit;
	incomparable!(Unit);

	#[derive(Debug)]
	#[derive_where(PartialEq, PartialOrd)]
	#[derive_where(incomparable)]
	struct Tuple();
	incomparable!(Tuple());

	#[derive(Debug)]
	#[derive_where(PartialEq, PartialOrd)]
	#[derive_where(incomparable)]
	struct Struct {}
	incomparable!(Struct {});
}

#[test]
fn enum_() {
	#[derive(Debug)]
	#[derive_where(PartialEq, PartialOrd)]
	#[derive_where(incomparable)]
	// Note that it is not possible to test an empty enum
	enum Empty {}

	#[derive(Debug)]
	#[derive_where(PartialEq, PartialOrd)]
	#[derive_where(incomparable)]
	enum Item {
		A,
		B,
	}
	incomparable!(Item::A);
	incomparable!(Item::A, Item::B);

	#[derive(Debug)]
	#[derive_where(PartialEq, PartialOrd)]
	enum Single {
		#[derive_where(incomparable)]
		A,
	}
	incomparable!(Single::A);

	#[derive(Debug)]
	#[derive_where(PartialEq, PartialOrd)]
	enum Enum {
		#[derive_where(incomparable)]
		IncomparableItem,
		#[derive_where(incomparable)]
		IncomparableTuple(u8),
		#[derive_where(incomparable)]
		IncomparableStruct {
			#[allow(unused)]
			a: u8,
		},
		ComparableItem,
		ComparableTuple(u8),
		ComparableStruct {
			a: u8,
		},
	}
	incomparable!(Enum::IncomparableItem);
	incomparable!(Enum::IncomparableItem, Enum::ComparableItem);
	incomparable!(Enum::IncomparableItem, Enum::ComparableTuple(1));
	incomparable!(Enum::IncomparableItem, Enum::ComparableStruct { a: 1 });

	incomparable!(Enum::IncomparableTuple(1));
	incomparable!(Enum::IncomparableTuple(1), Enum::IncomparableTuple(2));
	incomparable!(Enum::IncomparableTuple(1), Enum::ComparableItem);
	incomparable!(Enum::IncomparableTuple(1), Enum::ComparableTuple(1));
	incomparable!(Enum::IncomparableTuple(1), Enum::ComparableStruct { a: 1 });

	incomparable!(Enum::IncomparableStruct { a: 1 });
	incomparable!(
		Enum::IncomparableStruct { a: 1 },
		Enum::IncomparableStruct { a: 2 }
	);
	incomparable!(Enum::IncomparableStruct { a: 1 }, Enum::ComparableItem);
	incomparable!(Enum::IncomparableStruct { a: 1 }, Enum::ComparableTuple(1));
	incomparable!(
		Enum::IncomparableStruct { a: 1 },
		Enum::ComparableStruct { a: 1 }
	);

	assert_eq!(Enum::ComparableItem, Enum::ComparableItem);

	assert!(Enum::ComparableItem < Enum::ComparableTuple(0));
	assert_eq!(Enum::ComparableTuple(1), Enum::ComparableTuple(1));
	assert!(Enum::ComparableTuple(1) < Enum::ComparableTuple(2));

	assert!(Enum::ComparableTuple(1) < Enum::ComparableStruct { a: 0 });
	assert_eq!(
		Enum::ComparableStruct { a: 1 },
		Enum::ComparableStruct { a: 1 }
	);
	assert!(Enum::ComparableStruct { a: 1 } < Enum::ComparableStruct { a: 2 });
}
