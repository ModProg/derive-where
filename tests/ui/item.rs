extern crate derive_where as derive_where_;

use std::marker::PhantomData;

use derive_where::derive_where;

#[derive_where]
struct NoOption<T>(PhantomData<T>);

#[derive_where()]
struct EmptyAttribute<T>(PhantomData<T>);

#[derive_where(,)]
struct OnlyComma<T>(PhantomData<T>);

#[derive_where(crate(derive_where_))]
struct WrongCrateSyntax<T>(PhantomData<T>);

#[derive_where(crate = "struct Test")]
struct InvalidPathDeprecated<T>(PhantomData<T>);

#[derive_where(crate = struct Test)]
struct InvalidPath<T>(PhantomData<T>);

#[derive_where(skip_inner, Clone)]
struct SkipInnerWithTrait<T>(PhantomData<T>);

// The error message here shows that `crate = ..` should be in it's own
// attribute instead of an error pointing out this is duplicate. This is not
// ideal but much less complicated to implement.
#[derive_where(crate = derive_where_, crate = derive_where_)]
struct DuplicateCrate1<T>(PhantomData<T>);

#[derive_where(crate = derive_where_)]
#[derive_where(crate = derive_where_)]
struct DuplicateCrate2<T>(PhantomData<T>);

#[derive_where(crate = derive_where_)]
struct OnlyCrate<T>(PhantomData<T>);

#[derive_where(crate = ::derive_where)]
struct DefaultCrate<T>(PhantomData<T>);

#[derive_where(,Clone)]
struct StartWithComma<T>(PhantomData<T>);

#[derive_where(Clone,,)]
struct DuplicateCommaAtEnd<T>(PhantomData<T>);

#[derive_where("Clone")]
struct InvalidTrait<T>(PhantomData<T>);

#[derive_where(T)]
struct GenericInsteadTrait<T, U>(T, PhantomData<U>);

#[derive_where(Debug = invalid; T)]
struct WrongOptionSyntax<T, U>(T, PhantomData<U>);

#[derive_where(Clone; T;)]
struct SemiColonAtTheEnd<T, U>(T, PhantomData<U>);

#[derive_where(Clone; T,,)]
struct DoubleColonAtTheEnd<T, U>(T, PhantomData<U>);

#[derive_where(Clone; where)]
struct InvalidGeneric<T>(PhantomData<T>);

#[derive_where(Clone Debug)]
struct MissingCommaBetweenTraits<T>(PhantomData<T>);

#[derive_where(Clone; T U)]
struct MissingCommaBetweenGenerics<T, U, V>(T, PhantomData<(U, V)>);

#[derive_where(Clone)]
#[derive_where::derive_where(Copy)]
struct QualifiedNotFirstMacro<T>(PhantomData<T>);

#[derive_where(Clone, Clone)]
struct DuplicateTrait1<T>(PhantomData<T>);

#[derive_where(Clone)]
#[derive_where(Clone)]
struct DuplicateTrait2<T>(PhantomData<T>);

fn main() {}
