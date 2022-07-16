# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2022-07-16
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

[1.0.0]: https://github.com/ModProg/derive-where/compare/v1.0.0-rc.3...v1.0.0
[1.0.0-rc.3]: https://github.com/ModProg/derive-where/compare/v1.0.0-rc.2...v1.0.0-rc.3
[1.0.0-rc.2]: https://github.com/ModProg/derive-where/compare/v1.0.0-rc.1...v1.0.0-rc.2
[1.0.0-rc.1]: https://github.com/ModProg/derive-where/releases/tag/v1.0.0-rc.1
