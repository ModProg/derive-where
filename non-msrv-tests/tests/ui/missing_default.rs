#[derive(derive_where::DeriveWhere)]
#[derive_where(Default; T)]
enum Test<T> {
	A(T),
}

fn main() {}
