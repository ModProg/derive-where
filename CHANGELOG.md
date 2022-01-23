# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0-rc.2] - 2022-01-25
### Added
- Support [`ZeroizeOnDrop`](https://docs.rs/zeroize/1.5.0/zeroize/trait.ZeroizeOnDrop.html).

### Changed
- **Breaking Change**: Changed to attribute instead of derive proc macro.

### Removed
- **Breaking Change**: Remove support for `Zeroize(drop)`.

## [1.0.0-rc.1] - 2021-12-08
### Added
- Initial release.

[1.0.0-rc.2]: https://github.com/ModProg/derive-where/compare/v1.0.0-rc.1...v1.0.0-rc.2
[1.0.0-rc.1]: https://github.com/ModProg/derive-where/releases/tag/v1.0.0-rc.1
