extern crate zeroize_ as zeroize;

#[derive_where::derive_where(Zeroize(test); T)]
struct Test1<T>(T);

#[derive_where::derive_where(Zeroize(test = "test"); T)]
struct Test2<T>(T);

fn main() {}
