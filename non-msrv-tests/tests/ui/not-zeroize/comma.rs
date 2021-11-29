#[derive(derive_where::DeriveWhere)]
#[derive_where(Clone,; T)]
struct Test1<T>(T);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Clone,,)]
struct Test2<T>(T);

fn main() {}
