use derive_where::DeriveWhere;

#[derive(DeriveWhere)]
#[derive_where(Clone)]
struct StructEmpty {}

#[derive(DeriveWhere)]
#[derive_where(Clone)]
struct TupleEmpty();

#[derive(DeriveWhere)]
#[derive_where(Clone)]
struct Unit;

#[derive(DeriveWhere)]
#[derive_where(Clone)]
struct UnionEmpty {}

#[derive(DeriveWhere)]
#[derive_where(Clone)]
struct StructNone(u8);

#[derive(DeriveWhere)]
#[derive_where(Clone)]
enum EnumNone {
	A(u8),
}

#[derive(DeriveWhere)]
#[derive_where(Clone; T)]
struct SameGenerics<T>(T);

#[derive(DeriveWhere)]
#[derive_where(Clone, Debug; T)]
#[derive_where(skip_inner)]
struct Skip<T>(T);

#[derive(DeriveWhere)]
#[derive_where(Clone, Default; T)]
enum Default<T> {
	#[derive_where(default)]
	A(T),
}

fn main() {}
