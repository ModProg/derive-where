#[derive(derive_where::DeriveWhere)]
#[derive_where(Clone)]
struct Test1(u8);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Clone)]
enum Test2 {
    A,
}

// Make sure this doesn't reach unreachable code as it should fail early for
// not having any generics.
#[derive(derive_where::DeriveWhere)]
#[derive_where(Clone)]
struct Test3();

#[derive(derive_where::DeriveWhere)]
#[derive_where(Clone)]
enum Test4 {}

fn main() {}
