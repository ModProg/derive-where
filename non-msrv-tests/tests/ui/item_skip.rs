use std::marker::PhantomData;

use derive_where::DeriveWhere;

#[derive(DeriveWhere)]
#[derive_where(Debug; T)]
#[derive_where(skip_inner)]
enum SkipInnerOnEnum<T, U> {
	A(T),
	B(PhantomData<U>),
}

#[derive(DeriveWhere)]
#[derive_where(Debug; T)]
#[derive_where(skip_inner)]
#[derive_where(skip_inner)]
struct DuplicateSkipInner<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Debug; T)]
#[derive_where(skip_inner())]
struct EmptySkipInner<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Debug; T)]
#[derive_where(skip_inner)]
#[derive_where(skip_inner(Debug))]
struct OverridingSkipInner<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Clone; T)]
#[derive_where(skip_inner)]
struct NoSupportedTrait<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Clone; T)]
#[derive_where(skip_inner(Clone))]
struct UnsupportedTrait<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Debug; T)]
#[derive_where(skip_inner(Debug, Debug))]
struct DuplicateTraitSame<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Debug; T)]
#[derive_where(skip_inner(Debug))]
#[derive_where(skip_inner(Debug))]
struct DuplicateTraitSeparate<T, U>(T, PhantomData<U>);

#[derive(DeriveWhere)]
#[derive_where(Clone; T)]
#[derive_where(skip_inner(Debug))]
struct MissingDeriveTrait<T, U>(T, PhantomData<U>);

fn main() {}
