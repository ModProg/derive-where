extern crate zeroize_ as zeroize;

#[derive(derive_where::DeriveWhere)]
#[derive_where(Zeroize(test); T)]
struct Test1<T>(T);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Zeroize(test = "test"); T)]
struct Test2<T>(T);

fn main() {}
