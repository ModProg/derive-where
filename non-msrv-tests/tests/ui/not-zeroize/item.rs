use derive_where::derive_where;

#[derive_where(skip_inner, Clone)]
struct SkipInnerWithTrait<T>(PhantomData<T>);

#[derive_where(Debug = invalid; T)]
struct WrongOptionSyntax<T, U>(T, PhantomData<U>);

#[derive_where(,)]
struct OnlyComma<T>(PhantomData<T>);

#[derive_where(,Clone)]
struct StartWithComma<T>(PhantomData<T>);

#[derive_where(Clone,,)]
struct DuplicateCommaAtEnd<T>(PhantomData<T>);

#[derive_where(T)]
struct GenericInsteadTrait<T, U>(T, PhantomData<U>);

fn main() {}
