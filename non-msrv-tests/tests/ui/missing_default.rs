#[derive_where::derive_where(Default; T)]
enum Test<T> {
	A(T),
}

fn main() {}
