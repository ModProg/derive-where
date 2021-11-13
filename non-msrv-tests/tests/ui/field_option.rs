#[derive_where::derive_where(Default; T)]
enum Test<T> {
	#[derive_where(option)]
	A(T),
}

fn main() {}
