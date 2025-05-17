use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where(Debug)]
#[derive_where(skip_inner)]
enum SkipInnerOnEnum<T> {
	A(PhantomData<T>),
}

#[derive_where(Debug)]
#[derive_where(skip_inner)]
#[derive_where(skip_inner)]
struct DuplicateSkipInner<T>(PhantomData<T>);

#[derive_where(Debug)]
#[derive_where(skip_inner())]
struct EmptySkipInner<T>(PhantomData<T>);

#[derive_where(Debug)]
#[derive_where(skip_inner)]
#[derive_where(skip_inner(Debug))]
struct OverridingSkipInner<T>(PhantomData<T>);

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
#[derive_where(skip_inner)]
struct UnderridingSkipInner<T>(PhantomData<T>);

#[derive_where(Clone; T)]
#[derive_where(skip_inner)]
struct NoSupportedTrait<T>(PhantomData<T>);

#[derive_where(Copy; T)]
#[derive_where(skip_inner(Copy))]
struct UnsupportedTrait<T>(PhantomData<T>);

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug, Debug))]
struct DuplicateTraitSame<T>(PhantomData<T>);

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
#[derive_where(skip_inner(Debug))]
struct DuplicateTraitSeparate<T>(PhantomData<T>);

#[derive_where(Clone; T)]
#[derive_where(skip_inner(Debug))]
struct MissingDeriveTrait<T>(PhantomData<T>);

fn main() {}
