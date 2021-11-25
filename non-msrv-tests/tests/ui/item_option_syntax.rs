#[derive(derive_where::DeriveWhere)]
#[derive_where(Debug = "option"; T)]
struct Test<T>(T);

fn main() {}
