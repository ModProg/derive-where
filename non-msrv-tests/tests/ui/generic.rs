#[derive(derive_where::DeriveWhere)]
#[derive_where(Clone; where)]
struct Test<T>(T);

fn main() {}
