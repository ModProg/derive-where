#[derive_where::derive_where(Default; T)]
enum Test1<T> {
	#[derive_where(default, default)]
	A(T),
}

#[derive_where::derive_where(Default; T)]
enum Test2<T> {
	#[derive_where(default)]
	#[derive_where(default)]
	A(T),
}

fn main() {}
