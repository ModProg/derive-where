#![allow(unused)]
use std::marker::PhantomData;

use derive_where::derive_where;

/// Test for `clippy::non_canonical_clone_impl`, which should not be triggered
/// because of `#[automatically_derived]`.
#[derive_where(Clone, Copy; T: Copy)]
struct NonCanonicalCloneImpl<T>(PhantomData<T>);
