#[derive(derive_where::DeriveWhere)]
#[derive_where(Default; T)]
enum Test<T> {
	#[derive_where = "default"]
	A(T),
}

fn main() {}
