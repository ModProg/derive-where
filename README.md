# derive-where

[![Crates.io Version](https://img.shields.io/crates/v/derive-where.svg)](https://crates.io/crates/derive-where)
[![Live Build Status](https://img.shields.io/github/workflow/status/ModProg/derive-where/Test/main)](https://github.com/ModProg/derive-where/actions/workflows/test.yml)
[![Docs.rs Documentation](https://img.shields.io/docsrs/derive-where)](https://docs.rs/crate/derive-where)

## Description

Derive macro to simplify deriving standard and other traits with custom
generic type bounds.

## Usage

The `derive_where` macro can be used just like std's `#[derive(...)]`
statements:

```rust
#[derive_where(Clone, Debug)]
struct Example<T>(PhantomData<T>);
```

This will generate trait implementations for `Example` for any `T`,
as opposed to std's derives, which would only implement these traits with
`T: Trait` bound to the corresponding trait.

Multiple `derive_where` attributes can be added to an item, but only the
first one must use any path qualifications.

```rust
#[derive_where::derive_where(Clone)]
#[derive_where(Debug)]
struct Example1<T>(PhantomData<T>);
```

If using a different package name, you must specify this:

```rust
#[derive_where(crate = "derive_where_")]
#[derive_where(Clone, Debug)]
struct Example<T>(PhantomData<T>);
```

In addition, the following convenience options are available:

### Generic type bounds

Separated from the list of traits with a semi-colon, types to bind to can be
specified. This example will restrict the implementation for `Example` to
`T: Clone`:

```rust
#[derive_where(Clone; T)]
struct Example<T, U>(T, PhantomData<U>);
```

It is also possible to specify the bounds to be applied. This will
bind implementation for `Example` to `T: Super`:

```rust
trait Super: Clone {}

#[derive_where(Clone; T: Super)]
struct Example<T>(PhantomData<T>);
```

But more complex trait bounds are possible as well.
The example below will restrict the implementation for `Example` to
`T::Type: Clone`:

```rust
trait Trait {
	type Type;
}

struct Impl;

impl Trait for Impl {
	type Type = i32;
}

#[derive_where(Clone; T::Type)]
struct Example<T: Trait>(T::Type);
```

Any combination of options listed here can be used to satisfy a
specific constrain. It is also possible to use multiple separate
constrain specifications when required:

```rust
#[derive_where(Clone; T)]
#[derive_where(Debug; U)]
struct Example<T, U>(PhantomData<T>, PhantomData<U>);
```

### Enum default

Deriving [`Default`] on an enum is not possible in Rust at the moment.
Derive-where allows this with a `default` attribute:

```rust
#[derive_where(Default)]
enum Example<T> {
	#[derive_where(default)]
	A(PhantomData<T>),
}
```

### Skipping fields

With a `skip` or `skip_inner` attribute fields can be skipped for traits
that allow it, which are: [`Debug`], [`Hash`], [`Ord`](https://doc.rust-lang.org/core/cmp/trait.Ord.html), [`PartialOrd`](https://doc.rust-lang.org/core/cmp/trait.PartialOrd.html),
[`PartialEq`](https://doc.rust-lang.org/core/cmp/trait.PartialEq.html), [`Zeroize`] and [`ZeroizeOnDrop`].

```rust
#[derive_where(Debug, PartialEq; T)]
struct Example<T>(#[derive_where(skip)] T);

assert_eq!(format!("{:?}", Example(42)), "Example");
assert_eq!(Example(42), Example(0));
```

It is also possible to skip all fields in an item or variant if desired:

```rust
#[derive_where(Debug)]
#[derive_where(skip_inner)]
struct StructExample<T>(T);

assert_eq!(format!("{:?}", StructExample(42)), "StructExample");

#[derive_where(Debug)]
enum EnumExample<T> {
	#[derive_where(skip_inner)]
	A(T),
}

assert_eq!(format!("{:?}", EnumExample::A(42)), "A");
```

Selective skipping of fields for certain traits is also an option, both in
`skip` and `skip_inner`. To prevent breaking invariants defined for these
traits, some of them can only be skipped in groups. The following groups are
available:
- [`Debug`]
- `EqHashOrd`: Skips [`Eq`], [`Hash`], [`Ord`], [`PartialOrd`] and
  [`PartialEq`].
- [`Hash`]
- `Zeroize`: Skips [`Zeroize`] and [`ZeroizeOnDrop`].

```rust
#[derive_where(Debug, PartialEq)]
#[derive_where(skip_inner(Debug))]
struct Example<T>(i32, PhantomData<T>);

assert_eq!(format!("{:?}", Example(42, PhantomData::<()>)), "Example");
assert_ne!(
	Example(42, PhantomData::<()>),
	Example(0, PhantomData::<()>)
);
```

### `Zeroize` options

[`Zeroize`] has two options:
- `crate`: an item-level option which specifies a path to the `zeroize`
  crate in case of a re-export or rename.
- `fqs`: a field -level option which will use fully-qualified-syntax instead
  of calling the [`zeroize`][`method@zeroize`] method on `self` directly.
  This is to avoid ambiguity between another method also called `zeroize`.

```rust
#[derive_where(Zeroize(crate = "zeroize_"))]
struct Example(#[derive_where(Zeroize(fqs))] i32);

impl Example {
	// If we didn't specify the `fqs` option, this would lead to a compile
	//error because of method ambiguity.
	fn zeroize(&mut self) {
		self.0 = 1;
	}
}

let mut test = Example(42);

// Will call the struct method.
test.zeroize();
assert_eq!(test.0, 1);

// WIll call the `Zeroize::zeroize` method.
Zeroize::zeroize(&mut test);
assert_eq!(test.0, 0);
```

### `ZeroizeOnDrop` options

If the `zeroize-on-drop` feature is enabled, it implements [`ZeroizeOnDrop`]
and can be implemented without [`Zeroize`], otherwise it only implements
[`Drop`](https://doc.rust-lang.org/core/ops/trait.Drop.html) and requires [`Zeroize`] to be implemented.

[`ZeroizeOnDrop`] has one option:
- `crate`: an item-level option which specifies a path to the `zeroize`
  crate in case of a re-export or rename.

```rust
#[derive_where(ZeroizeOnDrop(crate = "zeroize_"))]
struct Example(i32);

assert!(core::mem::needs_drop::<Example>());
```

### Supported traits

The following traits can be derived with derive-where:
- [`Clone`](https://doc.rust-lang.org/core/clone/trait.Clone.html)
- [`Copy`](https://doc.rust-lang.org/core/marker/trait.Copy.html)
- [`Debug`]
- [`Default`]
- [`Eq`](https://doc.rust-lang.org/core/cmp/trait.Eq.html)
- [`Hash`]
- [`Ord`](https://doc.rust-lang.org/core/cmp/trait.Ord.html)
- [`PartialEq`](https://doc.rust-lang.org/core/cmp/trait.PartialEq.html)
- [`PartialOrd`](https://doc.rust-lang.org/core/cmp/trait.PartialOrd.html)
- [`Zeroize`]: Only available with the `zeroize` crate feature.
- [`ZeroizeOnDrop`]: Only available with the `zeroize` crate feature. If the
  `zeroize-on-drop` feature is enabled, it implements [`ZeroizeOnDrop`],
  otherwise it only implements [`Drop`](https://doc.rust-lang.org/core/ops/trait.Drop.html).

### Supported items

Structs, tuple structs, unions and enums are supported. Derive-where tries
it's best to discourage usage that could be covered by std's `derive`. For
example unit structs and enums only containing unit variants aren't
supported.

Unions only support [`Clone`](https://doc.rust-lang.org/core/clone/trait.Clone.html) and [`Copy`](https://doc.rust-lang.org/core/marker/trait.Copy.html).

### `no_std` support

`no_std` support is provided by default.

## Crate features

- `nightly`: Implements [`Ord`](https://doc.rust-lang.org/core/cmp/trait.Ord.html) and [`PartialOrd`](https://doc.rust-lang.org/core/cmp/trait.PartialOrd.html) with the help of
  [`core::intrinsics::discriminant_value`](https://doc.rust-lang.org/core/intrinsics/fn.discriminant_value.html), which is what Rust does by
  default too. Without this feature [`transmute`](https://doc.rust-lang.org/core/mem/fn.transmute.html) is
  used to convert [`Discriminant`](https://doc.rust-lang.org/core/mem/struct.Discriminant.html) to a [`i32`](https://doc.rust-lang.org/core/primitive.i32.html),
  which is the underlying type.
- `safe`: Implements [`Ord`](https://doc.rust-lang.org/core/cmp/trait.Ord.html) and [`PartialOrd`](https://doc.rust-lang.org/core/cmp/trait.PartialOrd.html) manually. This is much
  slower, but might be preferred if you don't trust derive-where. It also
  replaces all cases of [`core::hint::unreachable_unchecked`](https://doc.rust-lang.org/core/hint/fn.unreachable_unchecked.html) in [`Ord`](https://doc.rust-lang.org/core/hint/fn.unreachable_unchecked.html),
  [`PartialEq`](https://doc.rust-lang.org/core/cmp/trait.PartialEq.html) and [`PartialOrd`](https://doc.rust-lang.org/core/cmp/trait.PartialOrd.html), which is what std uses, with
  [`unreachable`](https://doc.rust-lang.org/core/macro.unreachable.html).
- `zeroize`: Allows deriving [`Zeroize`] and [`method@zeroize`] on [`Drop`](https://doc.rust-lang.org/core/ops/trait.Drop.html).
- `zeroize-on-drop`: Allows deriving [`Zeroize`] and [`ZeroizeOnDrop`] and
  requires [zeroize] v1.5.

## MSRV

The current MSRV is 1.57 and is being checked by the CI. A change will be
accompanied by a minor version bump. If MSRV is important to you, use
`derive-where = "~1.x"` to pin a specific minor version to your crate.

## Alternatives

[derivative](https://crates.io/crates/derivative)
([![Crates.io](https://img.shields.io/crates/v/derivative.svg)](https://crates.io/crates/derivative))
is a great alternative with many options. Notably it doesn't support `no_std`
and requires an extra `#[derive(Derivative)]` to use.

## Changelog

See the [CHANGELOG] file for details.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE] or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT] or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[CHANGELOG]: https://github.com/ModProg/derive-where/blob/main/CHANGELOG.md
[LICENSE-MIT]: https://github.com/ModProg/derive-where/blob/main/LICENSE-MIT
[LICENSE-APACHE]: https://github.com/ModProg/derive-where/blob/main/LICENSE-APACHE
[zeroize]: https://crates.io/crates/zeroize/1.5.2
[`Debug`]: https://doc.rust-lang.org/core/fmt/trait.Debug.html
[`Default`]: https://doc.rust-lang.org/core/default/trait.Default.html
[`Hash`]: https://doc.rust-lang.org/core/hash/trait.Hash.html
[`Zeroize`]: https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html
[`ZeroizeOnDrop`]: https://docs.rs/zeroize/1.5/zeroize/trait.ZeroizeOnDrop.html
[`method@zeroize`]: https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html#tymethod.zeroize
