extern crate zeroize_ as zeroize;

use derive_where::DeriveWhere;

#[derive(DeriveWhere)]
#[derive_where(Clone, Zeroize(crate = "zeroize_"); T)]
struct ZeroizeCrate<T>(T);

#[derive(DeriveWhere)]
#[derive_where(Clone, Zeroize; T)]
struct ZeroizeFqs<T>(#[derive_where(Zeroize(fqs))] T);

fn main() {}
