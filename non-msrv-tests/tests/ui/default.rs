use derive_where::derive_where;

#[derive_where(Debug)]
struct DefaultOnStruct<T>(#[derive_where(default)] PhantomData<T>);

#[derive_where(Clone)]
enum DefaultWithoutTrait<T> {
	#[derive_where(default)]
	A(PhantomData<T>),
}

#[derive_where(Default)]
enum MissingDefault<T> {
	A(PhantomData<T>),
}

#[derive_where(Default)]
enum DuplicateDefaultSeparate<T> {
	#[derive_where(default)]
	A(PhantomData<T>),
	#[derive_where(default)]
	B(PhantomData<T>),
}

#[derive_where(Default)]
enum DuplicateDefaultSame<T> {
	#[derive_where(default, default)]
	A(PhantomData<T>),
}

#[derive_where(Default)]
enum DuplicateDefaultSameSeparate<T> {
	#[derive_where(default)]
	#[derive_where(default)]
	A(PhantomData<T>),
}

fn main() {}
