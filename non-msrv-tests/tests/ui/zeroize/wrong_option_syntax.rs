#[derive(derive_where::DeriveWhere)]
#[derive_where(Zeroize("option"); T)]
struct Test1<T>(T);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Zeroize(crate(zeroize_)); T)]
struct Test2<T>(T);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Zeroize(crate = "struct Test"); T)]
struct Test3<T>(T);

fn main() {}
