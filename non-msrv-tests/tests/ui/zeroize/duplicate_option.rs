extern crate zeroize_ as zeroize;

#[derive(derive_where::DeriveWhere)]
#[derive_where(Zeroize(drop, drop); T)]
struct Test1<T>(T);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Zeroize(crate = "zeroize_", crate = "zeroize_"); T)]
struct Test2<T>(T);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Zeroize(drop, drop, crate = "zeroize_"); T)]
struct Test3<T>(T);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Zeroize(drop, crate = "zeroize_", crate = "zeroize_"); T)]
struct Test4<T>(T);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Zeroize(crate = "zeroize_", crate = "zeroize_", drop); T)]
struct Test5<T>(T);


#[derive(derive_where::DeriveWhere)]
#[derive_where(Zeroize(crate = "zeroize_", drop, drop); T)]
struct Test6<T>(T);

fn main() {}
