#[derive(derive_where::DeriveWhere)]
#[derive_where(Debug(option); T)]
struct Test1<T>(T);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Clone; T)]
#[derive_where(skip_inner)]
enum Test2<T> {
	A(T),
}

#[derive(derive_where::DeriveWhere)]
#[derive_where(Debug; T)]
#[derive_where(skip_inner)]
#[derive_where(skip_inner)]
struct Test3<T>(T);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Debug; T)]
#[derive_where(skip_inner())]
struct Test4<T>(T);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Debug; T)]
#[derive_where(skip_inner)]
#[derive_where(skip_inner(Debug))]
struct Test5<T>(T);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Clone; T)]
#[derive_where(skip_inner)]
struct Test6<T>(T);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Clone; T)]
#[derive_where(skip_inner(Clone))]
struct Test7<T>(T);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Debug; T)]
#[derive_where(skip_inner(Debug, Debug))]
struct Test8<T>(T);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Debug; T)]
#[derive_where(skip_inner(Debug))]
#[derive_where(skip_inner(Debug))]
struct Test9<T>(T);

#[derive(derive_where::DeriveWhere)]
#[derive_where(Clone; T)]
#[derive_where(skip_inner(Debug))]
struct Test10<T>(T);

fn main() {}
