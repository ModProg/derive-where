# derive-where

[![Crates.io Version](https://img.shields.io/crates/v/derive-where.svg)](https://crates.io/crates/derive-where)
[![Live Build Status](https://img.shields.io/github/workflow/status/ModProg/derive-where/Test/main)](https://github.com/ModProg/derive-where/actions/workflows/test.yml)
[![Docs.rs Documentation](https://img.shields.io/docsrs/derive-where)](https://docs.rs/crate/derive-where)

## Description

Derive macro to simplify deriving standard and other traits with custom
generic type bounds.

## Usage

The `derive_where` macro can be used just like std's `#[derive(...)]`
statements, with the only caveat that it requires to derive `DeriveWhere`
([#27]):

```rust
#[derive(DeriveWhere)]
#[derive_where(Clone, Debug)]
struct Example<T>(PhantomData<T>);
```

This will generate trait implementations for `Example` for any `T`,
as opposed to std's derives, which would only implement these traits with
`T: Trait` bound to the corresponding trait.

In addition, the following convenience options are available:

### Generic type bounds

Separated from the list of traits with a semi-colon, types to bind to can be
specified. This example will restrict the implementation for `Example` to
`T: Clone`:

```rust
#[derive(DeriveWhere)]
#[derive_where(Clone; T)]
struct Example<T, U>(T, PhantomData<U>);
```

It is also possible to specify the bounds to be applied. This will
bind implementation for `Example` to `T: Super`:

```rust
trait Super: Clone {}

#[derive(DeriveWhere)]
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

#[derive(DeriveWhere)]
#[derive_where(Clone; T::Type)]
struct Example<T: Trait>(T::Type);
```

Any combination of options listed here can be used to satisfy a
specific constrain. It is also possible to use multiple separate
constrain specifications when required:

```rust
#[derive(DeriveWhere)]
#[derive_where(Clone; T)]
#[derive_where(Debug; U)]
struct Example<T, U>(PhantomData<T>, PhantomData<U>);
```

### Enum default

Deriving [`Default`] on an enum is not possible in Rust at the moment.
Derive-where allows this with a `default` attribute:

```rust
#[derive(DeriveWhere)]
#[derive_where(Default)]
enum Example<T> {
	#[derive_where(default)]
	A(PhantomData<T>),
}
```

### Skipping fields

With a `skip` or `skip_inner` attribute fields can be skipped for traits
that allow it, which are: [`Debug`], [`Hash`], [`Ord`], [`PartialOrd`],
[`PartialEq`] and [`Zeroize`].

```rust
#[derive(DeriveWhere)]
#[derive_where(Debug, PartialEq; T)]
struct Example<T>(#[derive_where(skip)] T);

assert_eq!(format!("{:?}", Example(42)), "Example");
assert_eq!(Example(42), Example(0));
```

It is also possible to skip all fields in an item or variant if desired:

```rust
#[derive(DeriveWhere)]
#[derive_where(Debug)]
#[derive_where(skip_inner)]
struct StructExample<T>(T);

assert_eq!(format!("{:?}", StructExample(42)), "StructExample");

#[derive(DeriveWhere)]
#[derive_where(Debug)]
enum EnumExample<T> {
	#[derive_where(skip_inner)]
	A(T),
}

assert_eq!(format!("{:?}", EnumExample::A(42)), "A");
```

Selective skipping of fields for certain traits is also an option, both in
`skip` and `skip_inner`:

```rust
#[derive(DeriveWhere)]
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

[`Zeroize`] has three options:
- `crate`: an item-level option which specifies a path to the `zeroize`
  crate in case of a re-export or rename.
- `drop`: an item-level option which implements [`Drop`] and uses
  [`Zeroize`] to erase all data from memory.
- `fqs`: a field -level option which will use fully-qualified-syntax instead
  of calling the [`zeroize`][`method@zeroize`] method on `self` directly.
  This is to avoid ambiguity between another method also called `zeroize`.

```rust
#[derive(DeriveWhere)]
#[derive_where(Zeroize(crate = "zeroize_", drop))]
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

### Supported traits

The following traits can be derived with derive-where:
- [`Clone`]
- [`Copy`]
- [`Debug`]
- [`Default`]
- [`Eq`]
- [`Hash`]
- [`Ord`]
- [`PartialEq`]
- [`PartialOrd`]
- [`Zeroize`]: Only available with the `zeroize` crate feature.

### Supported items

Structs, tuple structs, unions and enums are supported. Derive-where tries
it's best to discourage usage that could be covered by std's `derive`. For
example unit structs and enums only containing unit variants aren't
supported.

Unions only support [`Clone`] and [`Copy`].

### `no_std` support

`no_std` support is provided by default.

## Crate features

- `nightly`: Implements [`Ord`] and [`PartialOrd`] with the help of
  [`core::intrinsics::discriminant_value`], which is what Rust does by
  default too. Without this feature [`transmute`] is used to convert
  [`Discriminant`] to a [`i32`], which is the underlying type.
- `safe`: Implements [`Ord`] and [`PartialOrd`] manually. This is much
  slower, but might be preferred if you don't trust derive-where. It also
  replaces all cases of [`core::hint::unreachable_unchecked`] in [`Ord`],
  [`PartialEq`] and [`PartialOrd`], which is what std uses, with
  [`unreachable`].
- `zeroize`: Allows deriving [`Zeroize`].

## MSRV

The current MSRV is 1.34 and is being checked by the CI. A change will be
accompanied by a minor version bump. If MSRV is important to you, use
`derive-where = "~1.x"` to pin a specific minor version to your crate.

## Alternatives

[derivative](https://crates.io/crates/derivative)
([![Crates.io](https://img.shields.io/crates/v/derivative.svg)](https://crates.io/crates/derivative))
is a great alternative with many options. Notably it has no `no_std`
support.

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
[`Clone`]: https://doc.rust-lang.org/core/clone/trait.Clone.html
[`Copy`]: https://doc.rust-lang.org/core/marker/trait.Copy.html
[`Debug`]: https://doc.rust-lang.org/core/fmt/trait.Debug.html
[`Default`]: https://doc.rust-lang.org/core/default/trait.Default.html
[`Drop`]: https://doc.rust-lang.org/core/ops/trait.Drop.html
[`Eq`]: https://doc.rust-lang.org/core/cmp/trait.Eq.html
[`Hash`]: https://doc.rust-lang.org/core/hash/trait.Hash.html
[`Ord`]: https://doc.rust-lang.org/core/cmp/trait.Ord.html
[`PartialOrd`]: https://doc.rust-lang.org/core/cmp/trait.PartialOrd.html
[`PartialEq`]: https://doc.rust-lang.org/core/cmp/trait.PartialEq.html
[`Zeroize`]: https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html
[`method@zeroize`]: https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html#tymethod.zeroize
[`core::hint::unreachable_unchecked`]: https://doc.rust-lang.org/core/hint/fn.unreachable_unchecked.html
[`core::intrinsics::discriminant_value`]: https://doc.rust-lang.org/core/intrinsics/fn.discriminant_value.html
[`Discriminant`]: https://doc.rust-lang.org/core/mem/struct.Discriminant.html
[`i32`]: https://doc.rust-lang.org/core/primitive.i32.html
[`transmute`]: https://doc.rust-lang.org/core/mem/fn.transmute.html
[`unreachable`]: https://doc.rust-lang.org/core/macro.unreachable.html
[#27]: https://github.com/ModProg/derive-where/issues/27
