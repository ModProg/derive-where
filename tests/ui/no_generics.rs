#[derive_where::derive_where(Clone)]
struct Test1(u8);

// Make sure this doesn't reach unreachable code as it should fail early for
// not having any generics.
#[derive_where::derive_where(Clone)]
struct Test2();

fn main() {}
