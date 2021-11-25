#[derive(derive_where::DeriveWhere)]
#[derive_where(Clone Debug; T)]
struct Test1<T>(T);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Clone; A B)]
struct Test2<A, B>(A, B);

fn main() {}
