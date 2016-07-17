# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]
### Breaking
- Remove `Builder::default` to force the specification of the C header file
  name.

### Added
- Add support for parsing complex macro definitions (integers only for now),
  see `--convert-macros` (#370)
- Support for `wchar_t`

### Changed
- Convert `float` and `double` to `f32` and `f64` by default (#348).

### Fixed
- Allow emission of -9223372036854775808 integer literal (#375)
- Pointers to function pointers now generate the correct number of
  indirections (#212)

## [0.18.0] - 2016-06-05
### Breaking
- New command line interface.
- Reserved words are suffixed and not prefixed (#302)
- `builder()` -> `Builder::new()`.

### Added
- `env_logger` for the bindgen binary.
- Possibility to configure the prefix for the C types in the generated code,
  instead of `std::os::raw`. See `--ctypes-prefix` (#316)
- Add the possibility to use `core` instead of `std` in the generated source.
  See `--use-core` (#316)
- Possibility to remove a prefix from all symbols, like `libfoo_`. See
  `--remove-prefix` (#93)

### Fixed
- Now correctly handle variable names like "u8".

## [0.17.0] - 2016-05-27
### Breaking
- Change the way to link to a library in the cli interface (see the `-l` option
  in the output of `--help`)
- Map stdint.h + size_t types to Rust native ones (#256)
- Default to fail on unknown type (see `-allow-unknown-types`)
- Convert C `typedef struct {} Test` to rust `struct Test {}` (#169)

### Added
- `-no-rust-enums` generate integer constants instead of enums
- Derive Debug when possible
- Support for inline enum declarations in struct fields (e.g.: `struct { enum { Option_A, Option_B } field; };`)
- Silent lint warnings (#112)

### Changed
- Use `clang_sys` instead of the internal ffi
- Use `[type; 0]` for the incomplete and dependent sized array
- Don't expand typedefed function parameter types (#212)
- Generate padding bytes in struct base on field align and struct size

### Fixed
- Fix build on OSX
- Fix overflow of enums variants (#232)
- Fix impl Clone on structs with large array member (#319)
- Fix function typedef generation when K&R style (no args specified) (#212)


## [0.16.0] - 2016-02-17
### Breaking
- Use `std::os::raw` instead of `libc`

### Added
- Translate C enums to Rust enums

### Fixed
- Various fixes

## [0.15.0] - 2016-08-31

[Unreleased]: https://github.com/crabtw/rust-bindgen/compare/0.17...HEAD
[0.18.0]: https://github.com/crabtw/rust-bindgen/compare/0.17...0.18
[0.17.0]: https://github.com/crabtw/rust-bindgen/compare/0.16...0.17
[0.16.0]: https://github.com/crabtw/rust-bindgen/compare/0.15...0.16
[0.15.0]: https://github.com/crabtw/rust-bindgen/compare/0.14...0.15
