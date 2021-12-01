#[derive(derive_where::DeriveWhere)]
#[derive_where(Clone)]
struct Test1(u8);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Clone)]
enum Test2 {
	A(u8),
}

fn main() {}
