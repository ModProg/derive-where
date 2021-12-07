use std::marker::PhantomData;

use derive_where::DeriveWhere;

#[derive(DeriveWhere)]
#[derive_where(Debug)]
struct DuplicateSkipSame<T>(#[derive_where(skip, skip)] PhantomData<T>);

#[derive(DeriveWhere)]
#[derive_where(Debug)]
struct DuplicateSkipSeparate<T>(
	#[derive_where(skip)]
	#[derive_where(skip)]
	PhantomData<T>,
);

#[derive(DeriveWhere)]
#[derive_where(Debug)]
struct EmptySkip<T>(#[derive_where(skip())] PhantomData<T>);

#[derive(DeriveWhere)]
#[derive_where(Debug)]
struct OverridingSkip<T>(
	#[derive_where(skip)]
	#[derive_where(skip(Debug))]
	PhantomData<T>,
);

#[derive(DeriveWhere)]
#[derive_where(Debug)]
struct UnderridingSkip<T>(
	#[derive_where(skip(Debug))]
	#[derive_where(skip)]
	PhantomData<T>,
);

#[derive(DeriveWhere)]
#[derive_where(Clone; T)]
struct NoSupportedTrait<T>(#[derive_where(skip)] PhantomData<T>);

#[derive(DeriveWhere)]
#[derive_where(Clone; T)]
struct UnsupportedTrait<T>(#[derive_where(skip(Clone))] PhantomData<T>);

#[derive(DeriveWhere)]
#[derive_where(Debug)]
struct DuplicateTraitSame<T>(#[derive_where(skip(Debug, Debug))] PhantomData<T>);

#[derive(DeriveWhere)]
#[derive_where(Debug)]
struct DuplicateTraitSeparate<T>(
	#[derive_where(skip(Debug))]
	#[derive_where(skip(Debug))]
	PhantomData<T>,
);

#[derive(DeriveWhere)]
#[derive_where(Clone; T)]
struct MissingDeriveTrait<T>(#[derive_where(skip(Debug))] PhantomData<T>);

fn main() {}
