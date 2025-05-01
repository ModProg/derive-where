# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.4.0] - 2025-05-01

### Added
- Allow skipping fields for `Clone`, calling `Default::default()` instead.
  **Note:** `Clone` is excluded from blanket skipping and can only be used with
  selective skipping to avoid this being a breaking change.

### Fixed
- Support skipping only some variants with `Zeroize`.

## [1.3.0] - 2025-04-21

### Added
- Allow lifetime `for<'a, ...>` bounds in non-bounded generic parameters.

### Changed
- Use the `Copy` implementation for `Clone` and the `Ord` implementation for
  `PartialOrd` when custom bounds are present.

### Fixed
- Support skipping only some variants with `ZeroizeOnDrop`.
- Only generate single call to `zeroize()` with `ZeroizeOnDrop` on multiple
  fields or variants.

## [1.2.7] - 2023-12-14

### Fixed
- Apply `#[automatically_derived]` to all generated implementations.

## [1.2.6] - 2023-12-04

### Fixed
- Use the `Copy` implementation for `Clone` only if no bounds are present.

## [1.2.5] - 2023-09-03

### Changed
- Use safe casting in `PartialOrd` and `Ord` implementations in more cases when
  possible.
- Avoid unnecessarily validating the default discriminant type in some cases.
- Apply default enum discriminant type validation to all representations and
  make it cross-edition safe.

## [1.2.4] - 2023-09-01

### Fixed
- Take representation into account when determining the discriminant type.

### Changed
- Use stable methods to retrieve the discriminant in `PartialOrd` and `Ord`
  implementations. Safe methods used with the `safe` crate feature don't use
  recursive matches anymore, significantly improving the performance.

## [1.2.3] - 2023-08-23 [YANKED]

### Fixed
- Don't use `Ord` in `PartialOrd` implementations if using any bounds.

## [1.2.2] - 2023-08-22 [YANKED]

### Fixed
- Avoid collisions between field names and trait method parameters.

### Changed
- `PartialOrd` implementations now use `Ord` if applicable.

## [1.2.1] - 2023-04-14 [YANKED]

### Fixed
- Correctly handle raw identifiers in named fields.

## [1.2.0] - 2023-03-19 [YANKED]

### Changed
- Updated `syn` to v2.
- `Debug` output of structs that skip fields appears `non_exhaustive` now.

### Deprecated
- The `crate` attribute now takes a bare path instead of a path inside a string
  literal.

## [1.1.0] - 2023-02-06 [YANKED]

### Added
- `incomparable` variant and item attribute for `PartialEq` and `PartialOrd`
  derives, yielding false on all comparisons but `!=`.

## [1.0.0] - 2022-07-16 [YANKED]
- No changes.

## [1.0.0-rc.3] - 2022-03-21

### Fixed
- Support attribute evaluation, e.g. `#[cfg(..)]` on fields.

### Changed
- **Breaking Change**: Upgraded MSRV to Rust 1.57.
- Upgraded to Rust edition 2021.
- **Breaking Change**: To prevent breaking invariants, skip groups were
  introduced: `Debug`, `EqHashOrd`, `Hash` and `Zeroize`.

## [1.0.0-rc.2] - 2022-01-25

### Added
- Support [`ZeroizeOnDrop`](https://docs.rs/zeroize/1.5/zeroize/trait.ZeroizeOnDrop.html).

### Changed
- **Breaking Change**: Changed to attribute instead of derive proc-macro.

### Removed
- **Breaking Change**: Remove support for `Zeroize(drop)`.

## [1.0.0-rc.1] - 2021-12-08

### Added
- Initial release.

[unreleased]: https://github.com/ModProg/derive-where/compare/v1.4.0...HEAD
[1.4.0]: https://github.com/ModProg/derive-where/compare/v1.3.0...v1.4.0
[1.3.0]: https://github.com/ModProg/derive-where/compare/v1.2.7...v1.3.0
[1.2.7]: https://github.com/ModProg/derive-where/compare/v1.2.6...v1.2.7
[1.2.6]: https://github.com/ModProg/derive-where/compare/v1.2.5...v1.2.6
[1.2.5]: https://github.com/ModProg/derive-where/compare/v1.2.4...v1.2.5
[1.2.4]: https://github.com/ModProg/derive-where/compare/v1.2.3...v1.2.4
[1.2.3]: https://github.com/ModProg/derive-where/compare/v1.2.2...v1.2.3
[1.2.2]: https://github.com/ModProg/derive-where/compare/v1.2.1...v1.2.2
[1.2.1]: https://github.com/ModProg/derive-where/compare/v1.2.0...v1.2.1
[1.2.0]: https://github.com/ModProg/derive-where/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/ModProg/derive-where/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/ModProg/derive-where/compare/v1.0.0-rc.3...v1.0.0
[1.0.0-rc.3]: https://github.com/ModProg/derive-where/compare/v1.0.0-rc.2...v1.0.0-rc.3
[1.0.0-rc.2]: https://github.com/ModProg/derive-where/compare/v1.0.0-rc.1...v1.0.0-rc.2
[1.0.0-rc.1]: https://github.com/ModProg/derive-where/releases/tag/v1.0.0-rc.1
