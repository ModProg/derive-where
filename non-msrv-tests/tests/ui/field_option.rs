#[derive(derive_where::DeriveWhere)]
#[derive_where(Clone; T)]
struct Test1<T>(#[derive_where(option)] T);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Clone; T)]
enum Test2<T> {
	#[derive_where(option)]
	A(T),
}

fn main() {}
