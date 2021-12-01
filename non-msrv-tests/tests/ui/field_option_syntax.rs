#[derive(derive_where::DeriveWhere)]
#[derive_where(Default; T)]
struct Test1<T>(#[derive_where] T);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Default; T)]
struct Test2<T>(#[derive_where = "default"] T);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Default; T)]
enum Test3<T> {
	#[derive_where]
	A(T),
}

#[derive(derive_where::DeriveWhere)]
#[derive_where(Default; T)]
enum Test4<T> {
	#[derive_where = "default"]
	A(T),
}

fn main() {}
