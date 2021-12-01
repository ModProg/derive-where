#[derive(derive_where::DeriveWhere)]
#[derive_where]
struct Test1<T>(T);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Debug = "option"; T)]
struct Test2<T>(T);

fn main() {}
