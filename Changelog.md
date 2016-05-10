# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]
### Added
- `-no-rust-enums` generate integer constants instead of enums
- Derive Debug when possible

### Changed
- Use `clang_sys` instead of the internal ffi

### Fixed
- Fix build on OSX
- Fix overflow of enums variants (#232)
- Fix impl Clone on structs with large array member (#319)


## [0.16.0] - 2016-02-17
### Breaking
- Use `std::os::raw` instead of `libc`

### Added
- Translate C enums to Rust enums

### Fixed
- Various fixes

## [0.15.0] - 2016-08-31

[Unreleased]: https://github.com/crabtw/rust-bindgen/compare/0.16...HEAD
[0.16.0]: https://github.com/crabtw/rust-bindgen/compare/0.15...0.16
[0.15.0]: https://github.com/crabtw/rust-bindgen/compare/0.14...0.15
