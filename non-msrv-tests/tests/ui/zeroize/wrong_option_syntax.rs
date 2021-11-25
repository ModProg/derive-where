#[derive(derive_where::DeriveWhere)]
#[derive_where(Zeroize(crate(zeroize_)); T)]
struct Test1<T>(T);

fn main() {}
