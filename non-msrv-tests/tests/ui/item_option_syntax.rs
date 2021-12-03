#[derive(derive_where::DeriveWhere)]
struct Test1<T>(T);

#[derive(derive_where::DeriveWhere)]
#[derive_where]
struct Test2<T>(T);

#[derive(derive_where::DeriveWhere)]
// Rust itself already fails to parse this and will provide a separate error message.
#[derive_where = invalid]
struct Test3<T>(T);

#[derive(derive_where::DeriveWhere)]
#[derive_where = "invalid"]
struct Test4<T>(T);

#[derive(derive_where::DeriveWhere)]
#[derive_where()]
struct Test5<T>(T);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Debug = "option"; T)]
struct Test6<T>(T);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Debug(); T)]
struct Test7<T>(T);

fn main() {}
