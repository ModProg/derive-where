#[derive(derive_where::DeriveWhere)]
#[derive_where(skip_inner, Clone)]
struct Test1<T>(core::marker::PhantomData<T>);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Debug = invalid; T)]
struct Test2<T>(T);

fn main() {}
