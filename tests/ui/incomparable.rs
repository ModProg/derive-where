use derive_where::derive_where;

#[derive_where(PartialEq)]
#[derive_where(incomparable)]
enum IncomparableOnVariantAndEnum {
	#[derive_where(incomparable)]
	Variant,
}

#[derive_where(Debug)]
#[derive_where(incomparable)]
enum IncomparableWithoutPartialCmpTrait {
	Variant,
}

#[derive_where(Eq)]
#[derive_where(incomparable)]
enum IncomparableWithCmpTrait {
	Variant,
}

fn main() {}
