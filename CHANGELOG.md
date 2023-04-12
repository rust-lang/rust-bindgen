<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->

- [Unreleased](#unreleased)
  - [Added](#added)
  - [Changed](#changed)
  - [Removed](#removed)
  - [Fixed](#fixed)
  - [Security](#security)
- [0.65.1](#0651)
  - [Fixed](#fixed-1)
- [0.65.0](#0650)
  - [Added](#added-1)
  - [Changed](#changed-1)
  - [Removed](#removed-1)
- [0.64.0](#0640)
  - [Added](#added-2)
  - [Changed](#changed-2)
- [0.63.0](#0630)
  - [Added](#added-3)
  - [Changed](#changed-3)
  - [Removed](#removed-2)
- [0.62.0](#0620)
  - [Added](#added-4)
  - [Changed](#changed-4)
  - [Fixed](#fixed-2)
- [0.61.0](#0610)
  - [Added](#added-5)
  - [Changed](#changed-5)
  - [Fixed](#fixed-3)
- [0.60.1](#0601)
  - [Fixed](#fixed-4)
- [0.60.0](#0600)
  - [Added](#added-6)
  - [Fixed](#fixed-5)
  - [Changed](#changed-6)
  - [Removed](#removed-3)
- [0.59.2](#0592)
- [0.59.1](#0591)
  - [Fixed](#fixed-6)
- [0.59.0](#0590)
  - [Added](#added-7)
  - [Fixed](#fixed-7)
  - [Changed](#changed-7)
- [0.58.1](#0581)
  - [Added](#added-8)
- [0.58.0](#0580)
  - [Added](#added-9)
  - [Fixed](#fixed-8)
  - [Changed](#changed-8)
  - [Deprecated](#deprecated)
  - [Removed](#removed-4)
  - [Fixed](#fixed-9)
  - [Security](#security-1)
- [0.57.0](#0570)
  - [Added](#added-10)
  - [Fixed](#fixed-10)
- [0.56.0](#0560)
  - [Added](#added-11)
  - [Changed](#changed-9)
  - [Fixed](#fixed-11)
- [0.55.1](#0551)
  - [Fixed](#fixed-12)
- [0.55.0](#0550)
  - [Removed](#removed-5)
  - [Added](#added-12)
  - [Changed](#changed-10)
  - [Fixed](#fixed-13)
- [0.54.1](#0541)
  - [Added](#added-13)
  - [Changed](#changed-11)
  - [Fixed](#fixed-14)
- [0.54.0](#0540)
  - [Added](#added-14)
  - [Changed](#changed-12)
  - [Fixed](#fixed-15)
- [0.53.3](#0533)
  - [Added](#added-15)
  - [Fixed](#fixed-16)
- [0.53.2](#0532)
  - [Changed](#changed-13)
- [0.53.1](#0531)
  - [Added](#added-16)
- [0.53.0](#0530)
  - [Added](#added-17)
  - [Changed](#changed-14)
  - [Fixed](#fixed-17)
- [0.52.0](#0520)
  - [Added](#added-18)
  - [Changed](#changed-15)
  - [Fixed](#fixed-18)
- [0.51.1](#0511)
  - [Fixed](#fixed-19)
  - [Changed](#changed-16)
- [0.51.0](#0510)
  - [Fixed](#fixed-20)
  - [Changed](#changed-17)
  - [Added](#added-19)
- [0.50.0](#0500)
  - [Added](#added-20)
- [0.49.3](#0493)
  - [Added](#added-21)
- [0.49.2](#0492)
  - [Changed](#changed-18)
- [0.49.1](#0491)
  - [Fixed](#fixed-21)
  - [Changed](#changed-19)
- [0.49.0](#0490)
  - [Added](#added-22)
  - [Fixed](#fixed-22)
  - [Changed](#changed-20)
- [0.48.1](#0481)
  - [Fixed](#fixed-23)
- [0.48.0](#0480)
  - [Changed](#changed-21)
  - [Fixed](#fixed-24)
- [0.47.4](#0474)
  - [Added](#added-23)
- [0.47.3](#0473)
  - [Changed](#changed-22)
- [0.47.2](#0472)
  - [Fixed](#fixed-25)
- [0.47.1](#0471)
  - [Changed](#changed-23)
  - [Fixed](#fixed-26)
- [0.47.0](#0470)
  - [Changed](#changed-24)
  - [Fixed](#fixed-27)
- [0.33.1 .. 0.46.0](#0331--0460)
  - [Added](#added-24)
  - [Removed](#removed-6)
  - [Changed](#changed-25)
  - [Fixed](#fixed-28)
- [0.33.1](#0331)
  - [Fixed](#fixed-29)
- [0.33.0](#0330)
- [0.32.2](#0322)
  - [Fixed](#fixed-30)
- [0.32.1](#0321)
  - [Fixed](#fixed-31)
- [0.32.0](#0320)
  - [Added](#added-25)
  - [Changed](#changed-26)
  - [Fixed](#fixed-32)
- [0.31.0](#0310)
  - [Added](#added-26)
  - [Changed](#changed-27)
  - [Deprecated](#deprecated-1)
  - [Removed](#removed-7)
  - [Fixed](#fixed-33)
- [0.30.0](#0300)
  - [Added](#added-27)
  - [Changed](#changed-28)
  - [Deprecated](#deprecated-2)
  - [Fixed](#fixed-34)
- [0.29.0](#0290)
  - [Added](#added-28)
  - [Changed](#changed-29)
  - [Fixed](#fixed-35)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

--------------------------------------------------------------------------------

# Unreleased

## Added

* Added the `--generate-cstr` CLI flag to generate string constants as `&CStr`
  instead of `&[u8]`. (Requires Rust 1.59 or higher.)

## Changed

* Non-UTF-8 string constants are now generated as references (`&[u8; SIZE]`)
  instead of arrays (`[u8; SIZE]`) to match UTF-8 strings.
* Wrappers for static functions that return `void` no longer contain a `return`
  statement and only call the static function instead.
* The `--wrap-static-fns` option no longer emits wrappers for static variadic
  functions.
* Depfiles generated with `--depfile` or `Builder::depfile` will now be
  properly generate module names and paths that include spaces by escaping
  them. To make the escaping clear and consistent, backslashes are also
  escaped.
   
## Removed

## Fixed

## Security

# 0.65.1

## Fixed

* The `Builder::rustfmt_bindings` method was added back and tagged as
  deprecated instead of being removed.
* Broken documentation links were fixed.

# 0.65.0

## Added
 * Added the `Builder::default_visibility` method and the
   `--default-visibility` flag to set the default visibility of fields. (#2338)
 * Added the `--formatter` CLI flag with the values `none`, `rustfmt` and
   `prettyplease` to select which tool will be used to format the bindings. The
   default value is `rustfmt`. (#2453)
 * Added the `Builder::formatter` method and the `Formatter` type to select
   which tool will be used to format the bindings. (#2453)
 * Added the `Builder::emit_diagnostics` method and the  `--emit-diagnostics`
   flag to enable emission of diagnostic messages under the `experimental`
   feature. (#2436)
 * Added support for the `"efiapi"` calling convention (#2490).
 * Added the `ParseCallbacks::read_env_var` method which runs everytime
   `bindgen` reads and environment variable. (#2400)
 * Added the `ParseCallbacks::generated_link_name_override` method which allow
   overriding the link name of items. (#2425)
 * Add support for C `enum`s when generating code while using the
   `--wrap-static-fns` feature. (#2415)

## Changed
 * Static functions with no arguments use `void` as their single argument
   instead of having no arguments when the `--wrap-static-fns` flag is used.
   (#2443)
 * The source file generated when the `--wrap-static-fns` flag is enabled now
   contains `#include` directives with all the input headers and all the source
   code added with the `header_contents` method. (#2447)
 * The source file generated when the `--wrap-static-fns` flag no longer uses
   `asm` labeling and the link name of static wrapper functions is allowed to
   be mangled. (#2448)
 * The documentation of the generated `type` aliases now matches the comments
   of their `typedef` counterparts instead of using the comments of the aliased
   types. (#2463)
 * The `Builder::rustfmt_bindings` methods and the `--no-rustfmt-bindings` flag
   are now deprecated in favor of the formatter API. (#2453)
   
## Removed
 * The following deprecated flags were removed: `--use-msvc-mangling`,
   `--rustfmt-bindings` and `--size_t-is-usize`. (#2408)
 * The `Bindings::emit_warnings` and `Bindings::warnings` methods were removed
   in favor of `--emit-diagnostics`. (#2436)
 * Bindgen no longer generates C string constants that cannot be represented as
   byte slices. (#2487)

# 0.64.0

## Added
 * Added a new set of flags `--with-derive-custom`,
   `--with-derive-custom-struct`, `--with-derive-custom-enum` and
   `--with-derive-custom-enum` to add custom derives from the CLI.
 * Added the `--experimental` flag on `bindgen-cli` and the `experimental`
   feature on `bindgen` to gate experimental features whose implementation is
   incomplete or are prone to change in a non-backwards compatible manner.
 * Added a new set of flags and their equivalent builder methods
   `--wrap-static-fns`, `--wrap-static-fns-suffix` and `--wrap-static-fns-path`
   to generate C function wrappers for `static` or `static inline` functions.
   This feature is experimental.

## Changed
 * Fixed name collisions when having a C `enum` and a `typedef` with the same
   name.
 * The `ParseCallbacks::generated_name_override` method now receives `ItemInfo<'_>` as
   argument instead of a `&str`.
 * Updated the `clang-sys` crate version to 1.4.0 to support clang 15.
 * The return type is now ommited in signatures of functions returning `void`.
 * Updated the `clap` dependency for `bindgen-cli` to 4.
 * Rewrote the `bindgen-cli` argument parser which could introduce unexpected
   behavior changes.
 * The `ParseCallbacks::add_derives` method now receives `DeriveInfo<'_>` as
   argument instead of a `&str`. This type also includes the kind of target type.

# 0.63.0

## Added
 * new feature: `process_comments` method to the `ParseCallbacks` trait to
   handle source code comments.

## Changed
 * Only wrap unsafe operations in unsafe blocks if the `--wrap_unsafe_ops`
   option is enabled.
 * Replace the `name: &str` argument for `ParseCallbacks::add_derives` by
   `info: DeriveInfo`.
 * All the rust targets equal or lower than `1.30` are being deprecated and
   will be removed in the future. If you have a good reason to use any of these
   targets, please report it in the issue tracker.

## Removed

 * The following deprecated methods and their equivalent CLI arguments were
   removed: `whitelist_recursively`, `hide_type`, `blacklist_type`,
   `blacklist_function`, `blacklist_item`, `whitelisted_type`,
   `whitelist_type`, `whitelist_function`, `whitelisted_function`,
   `whitelist_var`, `whitelisted_var`, `unstable_rust`.

# 0.62.0

## Added

 * new feature: `--override-abi` flag to override the ABI used by functions
   matching a regular expression.
 * new feature: allow using the `C-unwind` ABI in `--override-abi` on nightly
   rust.

## Changed

 * Regex inputs are sanitized so alternation (`a|b`) is handled correctly but
   wildcard patterns (`*`) are now considered invalid. The `.*` pattern can be
   used as a replacement.
 * the `ParseCallbacks`trait does not require to implement `UnwindSafe`.
 * the `Builder::parse_callbacks` method no longer overwrites previously added
   callbacks and composes them in a last-to-first manner.
 * any generated rust code containing unsafe operations inside unsafe functions
   is wrapped in unsafe blocks now.

## Fixed

 * Various issues with upcoming clang/libclang versions have been fixed.

# 0.61.0

Released 2022/10/16

## Added

 * new feature: `--sort-semantically` flag to sort the output in a predefined
   manner [(#1743)].
 * new feature: `Bindgen::emit_warnings` method to emit warnings to stderr in
   build scripts.
 * new feature: `--newtype-global-enum` flag to generate enum variants as
   global constants.
 * new feature: `--default-non-copy-union-style` flag to set the default style
   of code used to generate unions with non-`Copy` members.
 * new feature: `--bindgen-wrapper-union` flag to mark any union that matches a
   regex and has a non-Copy member to use a bindgen-generated wrapper for its
   fields.
 * new feature: `--manually-drop-union` flag to mark any union that matches a
   regex and has a non-`Copy` member to use `ManuallyDrop`.
 * new feature: `--merge-extern-blocks` flag to merge several `extern` blocks
   that have the same ABI.
 * new feature: `--no-size_t-is-usize` flag to not bind `size_t` as `usize`.
 * new feature: `Builder` implements `Clone`.

## Changed

 * clap and regex have been updated, new msrv is 1.57.
 * The `--enable-function-attribute-detection` flag is also used to detect
   diverging functions so the generated bindings use `!` as the return type.
 * The `--size_t-is-usize` flag is enabled by default.
 * Unused type aliases for `<stdint.h>` types are no longer emitted.
 * The `blocklist` options now can be used to block objective-C methods.
 * The `core::ffi` module is used the sized raw integer types
   instead of `std::os::raw` if the Rust target version is `1.64` or higher and
   the `--use-core` flag is enabled.
 * The `bindgen` CLI utility must be installed using `cargo install
   bindgen-cli` now.
 * Using `bindgen` as a library no longer pulls clap and any other CLI
   related dependencies.

## Fixed

 * Const correctness of incomplete arrays has been fixed. (#2301)
 * C++ inline namespaces don't panic. (#2294)

[(#1743)]: https://github.com/rust-lang/rust-bindgen/issues/1743

# 0.60.1

Released 2022/06/06

## Fixed

 * Fixed stack overflow in generated tests for structs with many fields (#2219).

# 0.60.0

Released 2022/06/05

## Added

 * Objective-C structs now derive `Debug` and `Copy` to support C and Objective-C structs. [(#2176)][]
 * Allow fully-qualified derives. (#2156)
 * Bindings generation now returns a more suitable error (#2125)
 * `--version --verbose` now prints clang version (#2140).
 * Experimental vtable generation (#2145).
 * Added an `--allowlist-file` option (#2122).
 * Support for vectorcall ABI (#2177).

## Fixed

 * Fixed lifetimes with Objective-C trait templates. [(#2176)][]
 * Fixed objc imports for non-`#[macro_use]` use. [(#2176)][]
 * Handle differences between clang and rustc targets for RISCV (#2137).
 * `BINDGEN_EXTRA_CLANG_ARGS` is respected on the CLI now (#1723).
 * Use common type alias for anonymous enums in consts mode (#2191)
 * Look for `#[must_use]` in typedefs (#2206).
 * Fixed derive on packed structs (#2083).
 * Fixed warnings on layout tests (#2203).

## Changed

 * cexpr, clap, and nom have been updated, new msrv is 1.54.

## Removed

 * Support for ancient libclang versions has been removed.

 [(#2176)]: https://github.com/rust-lang/rust-bindgen/pull/2176

# 0.59.2

Released 2021/11/26

 * cexpr+env_logger bump.
 * Various fixes for C++ crashes / hangs.
 * Enums now respect annotations and derives properly in more cases.
 * Some more APIs (blocklist-file, etc).
 * 'static lifetime is elided when appropriate.

# 0.59.1

Released 2021/07/26

## Fixed

 * Fixed incorrect bitfield constructors generated for very large bitfields (#2082).

# 0.59.0

Released 2021/07/20

## Added

 * Support emitting Makefile-syntax depfiles (#2026)
 * Add a C naming option (#2045)
 * Allow explicit padding (#2060)
 * Add custom derives callback (#2059)
 * Let Rust derive everything but Default for large arrays in 1.47 and later (#2070).

## Fixed

 * Constants now have docstrings (#2027)
 * Don't generate bindings for deleted member functions. (#2044)
 * Zero out padding in custom Default trait implementations (#2051)
 * Identify forward declarations in params. (#2052)
 * Add env var EXTRA_CLANG_ARGS_<target>. (#2031)

## Changed

 * cexpr and nom have been updated, new msrv is 1.44 (#2073).

# 0.58.1

Released 2021/04/06

## Added

 * Re-introduced unintentionally removed
   `bindgen::Builder::whitelist_recursively` (deprecated in favor of
  `bindgen::Builder::allowlist_recursively`). [#2022][]

# 0.58.0

Released 2021/04/03

## Added

 * Add option to translate enum integer types to native Rust integer types.
   [#2004][]
 * Add callback to check derives for blocklisted types. [#2007][]
 * Add a flag to ensure all symbols are resolved when a library is loaded.
   [#2013][]
 * Add from_library for generated dynamic library structs [#2011][].

## Fixed

 * Track union layout more accurately. Fixes [an AArch64 bug] and [makes the
   bindings more portable] where unions could return garbage data ([#1984])
 * Use original name when checking allowlist for anonymous enum variants. [#2006][]

## Changed

## Deprecated

* `bindgen::Builder::whitelist_type` is deprecated in favor of
  `bindgen::Builder::allowlist_type`. [#1812][]

* `bindgen::Builder::whitelist_function` is deprecated in favor of
  `bindgen::Builder::allowlist_function`. [#1812][]

* `bindgen::Builder::whitelist_var` is deprecated in favor of
  `bindgen::Builder::allowlist_var`. [#1812][]

* `--whitelist-type` is deprecated in favor of
  `--allowlist-type`. [#1812][]

* `--whitelist-function` is deprecated in favor of
  `--allowlist-function`. [#1812][]

* `--whitelist-var` is deprecated in favor of
  `--allowlist-var`. [#1812][]

* `bindgen::Builder::blacklist_type` is deprecated in favor of
  `bindgen::Builder::blocklist_type`. [#1812][]

* `bindgen::Builder::blacklist_function` is deprecated in favor of
  `bindgen::Builder::blocklist_function`. [#1812][]

* `bindgen::Builder::blacklist_item` is deprecated in favor of
  `bindgen::Builder::blocklist_item`. [#1812][]

* `--blacklist-type` is deprecated in favor of
  `--blocklist-type`. [#1812][]

* `--blacklist-function` is deprecated in favor of
  `--blocklist-function`. [#1812][]

* `--blacklist-item` is deprecated in favor of
  `--blocklist-item`. [#1812][]

[#1984]: https://github.com/rust-lang/rust-bindgen/pull/1984
[an AArch64 bug]: https://github.com/rust-lang/rust-bindgen/issues/1973
[makes the bindings more portable]: https://github.com/rust-lang/rust-bindgen/issues/1983

## Removed

## Fixed

## Security

---

# 0.57.0

Released 2021/02/01

## Added

* Expose module-raw-lines to the CLI (#1936)
* Added an option to fit macro constants to smaller types (#1945)
* Add an option to respect C++ access specifiers on fields (#1968)

## Fixed

* Improved C++ auto-detection (#1933)
* Fixed layout of bitfields in some edge cases (#1950)
* Escape the dyn keyword properly (#1951)
* Use absolute paths for unsaved files passed to clang (#1857).

# 0.56.0

Released 2020/11/26

## Added

* Objective-c bindings generate `From<ChildClass> for ParentClass` as well as `TryFrom<ParentClass> for ChildClass` ([#1883][]).
* Experimental dynamic library support via `dynamic_library_name` (#1846).
* Option to disable deriving `Default` on a per-struct basis (#1930).

## Changed

* Objective-c bindings borrow self rather than take ownership ([#1883][]).
* Templates and enums now correctly use the same naming scheme as other types
 (#1891).

## Fixed

* Constructors in wasm32 now return a value. (#1877).
* Fixed objective-c protocol impl blocks for parent classes's protocols ([#1883][]).

[#1883]: https://github.com/rust-lang/rust-bindgen/issues/1883

--------------------------------------------------------------------------------

# 0.55.1

Released 2020/08/24.

## Fixed

 * Fixed a regression where anonymous enums referenced by members or such won't
   generate valid Rust code. (#1882).

--------------------------------------------------------------------------------

# 0.55.0

Released 2020/08/23.

## Removed

 * Support for libclang 3.8 has been removed (#1830).

## Added

 * Added options to avoid deriving the Debug trait (#1858).

 * Added options to allow to override the default anonymous field prefix (#1859).

 * Added options to allow to override the default macro integer type from the
   command line (#1863).

## Changed

 * Typed anonymous enums now generate better code (#1850).

 * Objective-C bindings are more idiomatic now (#1847).

 * Updated to clang-sys 1.0. Minimum supported rust version is 1.40 as
   a consequence of that change.

## Fixed

 * Fixed constness of multi-dimensional arrays in some cases (#1861).

 * Fixed wrong target given to clang when compiling with a target which doesn't
   match the target clang expects (#1870, #1878).

 * Fixed wrong flags being computed for cross-compilation cases where the target
   wasn't explicitly provided via clang flags (#1872).

Thanks again to all the awesome contributors that sent patches included in this
release!

--------------------------------------------------------------------------------

# 0.54.1

Released 2020/07/06.

**Yanked**: The change in #1798 is technically breaking, see PR for details.

## Added

 * Added ParseCallbacks::func_macro to be able to process function-like macros.
   (#1792).

 * Allowed IntKind::Custom to represent paths instead of idents (#1800).

## Changed

 * Generated comment now includes the bindgen version, and can be disabled
   (#1814).

 * Various documentation improvements.

## Fixed

 * Typedefs for types with the same names as rust primitive types compiles
   (#1798).

 * Bindgen dependencies will now get rebuilt when various environment variables
   that affect bindgen change (#1809, #1813).

 * Various fixes to command_line_flags (#1816, #1819, #1821).

 * Functions that start with `operator` now get properly generated (#1817).


Thanks to all the awesome contributors that sent patches included in this
release!

--------------------------------------------------------------------------------

# 0.54.0

Released 2020/05/21.

## Added

 * New command line flag to allow disabling untagged unions (#1789).

## Changed

 * Various documentation improvements (#1764, #1751, #1757).
 * Better Objective-C support (#1722, #1750).

## Fixed

 * Rust method wrappers are not emitted for blacklisted functions (#1775).
 * Fixed function signatures in some edge cases involving Objective-C or
   `__stdcall` (#1781).

--------------------------------------------------------------------------------

# 0.53.3

Released 2020/05/21.

*Note: This release contains the same fixes and additions as 0.54.0, but without
the Objective-C breaking changes*

## Added

 * New command line flag to allow disabling untagged unions (#1789).

## Fixed

 * Rust method wrappers are not emitted for blacklisted functions (#1775).
 * Fixed function signatures in some edge cases involving Objective-C or
   `__stdcall` (#1781).

--------------------------------------------------------------------------------

# 0.53.2

Released 2020/03/10.

## Changed

 * clang-sys and cexpr have been updated (#1741 and #1744).
 * Runtime of some commands has been improved (#1737)
 * Some error messages have been improved (#1734).

--------------------------------------------------------------------------------

# 0.53.1

Released 2020/02/03.

## Added

 * Opt-in to convert size_t to usize again (#1720).

--------------------------------------------------------------------------------

# 0.53.0

Released 2020/02/02.

## Added

 * Support for wasm_import_module. (#1691).
 * non_exhaustive feature is now stable (#1698).
 * Various objective-C improvements (#1702).

## Changed

 * Removed size_t to usize conversion rule (#1688).

## Fixed

 * Various unneeded padding fields shouldn't be generated anymore (#1710).
 * Bitfields on packed structs should generate correct layout (#1717).
 * Too large bitfield blocks now generate compiling code (#1719).

--------------------------------------------------------------------------------

# 0.52.0

Released 2019/11/19.

## Added

 * Added `newtype` enum style, much like `bitfield` but without the bitwise ops
   (#1677).
 * Added support for `MaybeUninit` rather than `mem::uninitialized()` (#1666).
 * Allowed static linking (#1620) behind a feature. Note that **if you're using
   `default-features = false`, you probably want to use the `"runtime"` feature
   to get the same behavior as before**.

## Changed

 * Use c_void from core when --use-core is specified and available (#1634).
 * Various dependencies and features are non-default now (like `regex` unicode
   features).

## Fixed

 * Fixed crash when unknown keywords are used before a namespace (#1678).
 * Do not generate implementation for clone for flexible array members (#1664).
 * Fixed `#[must_use]` support for libclang 9+ (#1646).
 * Fixed `BitfieldUnit` constructor to handle 64 bit wide bitfields on 32 bit (#1640).
 * Added a `ParseCallbacks` handler for included files. (#1637).

# 0.51.1

Released 2019/09/23.

## Fixed

 * Mismatched `Ord` and `PartialOrd` implementations were fixed, which regresses
   bindgen in funny ways when using rustc nightly. Dot releases for a few of the
   previous versions of bindgen will be created with this fix. Also,
   a `v0.51.1-oldsyn` version was uploaded without the syn update. [#1627][]

## Changed

 * Syn and related dependencies have been updated. [#1611][]

 * Switches added to allow less dependencies. In
   particular: It won't pull `failure` and related dependencies by default, and
   there's a default-on `which-rustfmt` feature which allows to get rid of
   `which` altogether. [#1615][] / [#1625][]

 * `fxhash` dependency was switched to `rustc-hash`. [#1626][]

[#1627]: https://github.com/rust-lang/rust-bindgen/issues/1627
[#1611]: https://github.com/rust-lang/rust-bindgen/issues/1611
[#1615]: https://github.com/rust-lang/rust-bindgen/issues/1615
[#1625]: https://github.com/rust-lang/rust-bindgen/issues/1625
[#1626]: https://github.com/rust-lang/rust-bindgen/issues/1626
[#1627]: https://github.com/rust-lang/rust-bindgen/issues/1627

# 0.51.0

Released 2019/07/26.

## Fixed

 * Improve workaround for LLVM stack overflow when evaluating value-dependent
   expressions. [#1591][]

 * Bindgen will properly detect the layout of incomplete arrays. [#1592][]

 * Bindgen will properly detect the layout of empty unions and forward
   declarations of unions. [#1593][] and [#1595][]. Thanks @pmarks!

## Changed

 * Refactored the way layout of `wchar_t` is computed. This is a breaking change
   since `IntKind::WChar` (exposed in `ParseCallbacks`) no longer needs a `size`
   member. [#1596][]

## Added

 * Bindgen now reads `RUSTFMT` in the environment to try to find a suitable
   `rustfmt` binary. [#1602][]

[#1591]: https://github.com/rust-lang/rust-bindgen/issues/1591
[#1592]: https://github.com/rust-lang/rust-bindgen/issues/1592
[#1593]: https://github.com/rust-lang/rust-bindgen/issues/1593
[#1595]: https://github.com/rust-lang/rust-bindgen/issues/1595
[#1596]: https://github.com/rust-lang/rust-bindgen/issues/1596
[#1602]: https://github.com/rust-lang/rust-bindgen/issues/1602

# 0.50.0

Released 2019/07/01.

## Added

* Fixed pointers to Objective C blocks [#1582][].

* Various bindgen auto-generated types are now constructible in `const fn`
  contexts [#1571][]

* It is possible to generate `#[non_exhaustive]` enums for rust nightly targets.
  [#1575][]

* It is possible to avoid building clap now if you're using bindgen as
  a library. [#1581][].

[#1571]: https://github.com/rust-lang/rust-bindgen/issues/1571
[#1575]: https://github.com/rust-lang/rust-bindgen/issues/1575
[#1581]: https://github.com/rust-lang/rust-bindgen/issues/1581
[#1582]: https://github.com/rust-lang/rust-bindgen/issues/1582

# 0.49.3

Released 2019/06/25. **YANKED**

## Added

* Various bindgen auto-generated types are now constructible in `const fn`
  contexts [#1571][]

* It is possible to generate `#[non_exhaustive]` enums for rust nightly targets.
  [#1575][]

* It is possible to avoid building clap now if you're using bindgen as
  a library. [#1581][].

[#1571]: https://github.com/rust-lang/rust-bindgen/issues/1571
[#1575]: https://github.com/rust-lang/rust-bindgen/issues/1575
[#1581]: https://github.com/rust-lang/rust-bindgen/issues/1581

# 0.49.2

Released 2019/05/22

## Changed

* Bindgen now has an option to generate array arguments as pointer to the array,
  not to the element (so `void foo(int arr[2])` would be generated as
  `arr: *mut [c_int; 2]` rather than `arr: *mut c_int`. Thanks @elichai!
  [#1564][].

[#1564]: https://github.com/rust-lang/rust-bindgen/issues/1564

# 0.49.1

Released 2019/05/16

## Fixed

* Bindgen will not emit `#[link_name]` attributes in win32 and macos for
  C functions and constants where it can detect it's not needed (thanks
  @michaelwoerister!). [#1558][]

## Changed

* Bindgen will no longer use `hashbrown` internally, and will use fxhash
  and `std::HashMap`. This is equivalent for newer `rustc`s since `hashbrown`
  was merged in libstd, and the performance difference should be close to zero
  for older rustcs.

[#1558]: https://github.com/rust-lang/rust-bindgen/issues/1558

# 0.49.0

Released 2019/03/27

## Added

* BINDGEN_EXTRA_CLANG_ARGS environment variable was added (thanks @jhwgh1968!). [#1537][]

## Fixed

* Bindgen will properly name parameters inside nested function pointer
  declarations (thanks @flowbish!). [#1535][]

## Changed

* Derive code was greatly improved by @jethrogb. [#1540][]
* Derive analysis now handles trivial types more gracefully. [#1492][]
* clang-sys was updated by @eclipseo. [#1539][]
* bindgen should now get include paths correctly even when `--target` is
  specified. The `detect_include_paths` option can be used to opt-out of this
  behavior.

[#1535]: https://github.com/rust-lang/rust-bindgen/issues/1535
[#1537]: https://github.com/rust-lang/rust-bindgen/issues/1537
[#1540]: https://github.com/rust-lang/rust-bindgen/issues/1540
[#1492]: https://github.com/rust-lang/rust-bindgen/issues/1492

# 0.48.1

Released 2019/03/06

## Fixed

* Bindgen will properly lay out types that use reference members. [#1531][]

[#1531]: https://github.com/rust-lang/rust-bindgen/issues/1531

--------------------------------------------------------------------------------

# 0.48.0

Released 2019/03/04

## Changed

* Default rust target was changed to 1.33, which means that bindgen can get much
  more often the layout of structs right. [#1529][]

## Fixed

* Bindgen will output repr(align) just when needed for unions. [#1498][]

[#1529]: https://github.com/rust-lang/rust-bindgen/issues/1529
[#1498]: https://github.com/rust-lang/rust-bindgen/issues/1498

--------------------------------------------------------------------------------

# 0.47.4

Released 2020/11/13

## Added

* Backported BINDGEN_EXTRA_CLANG_ARGS support per request (#1910).

--------------------------------------------------------------------------------

# 0.47.3

Released 2019/02/25

## Changed

* Allowed to build with which 1.0.

--------------------------------------------------------------------------------

# 0.47.2

Released 2019/02/22

## Fixed

* @flowbish fixed code generation for nested function prototypes. [#1508][]
* Some complex C++ constructs no longer panic on code generation [#1513][]
* Implicit template parameters are now appended to base classes [#1515][]
* @flier fixed single-argument block pointers [#1519][]
* Bindgen won't panic when parsing an undeduced auto type [#1525][]

[#1508]: https://github.com/rust-lang/rust-bindgen/issues/1508
[#1513]: https://github.com/rust-lang/rust-bindgen/issues/1513
[#1515]: https://github.com/rust-lang/rust-bindgen/issues/1515
[#1519]: https://github.com/rust-lang/rust-bindgen/issues/1519
[#1525]: https://github.com/rust-lang/rust-bindgen/issues/1525

--------------------------------------------------------------------------------

# 0.47.1

Released 2019/02/02

## Changed

* @luser improved the error message when rustfmt cannot be found [#1501][]

## Fixed

* Reverted `clang-sys` update for regressions [#1505][]

[#1505]: https://github.com/rust-lang/rust-bindgen/issues/1505
[#1501]: https://github.com/rust-lang/rust-bindgen/issues/1501

--------------------------------------------------------------------------------

# 0.47.0

Released 2019/01/19

## Changed

- `#pragma pack(n)` is now translated to `#[repr(C, packed(n))]` when targeting Rust 1.33+. [#537][]

[#537]: https://github.com/rust-lang/rust-bindgen/issues/537

* Bitfield enums now use `#[repr(transparent)]` instead of `#[repr(C)]` when targeting Rust 1.28+. [#1474][]

[#1474]: https://github.com/rust-lang/rust-bindgen/issues/1474

## Fixed

* `#[repr(packed)]` is now properly added if the struct only contains a vtable.
  [#1495][]

[#1495]: https://github.com/rust-lang/rust-bindgen/pull/1495

* `clang-sys` should now more accurately find libclang versions when multiple
  of them are available. [#1489][]

[#1489]: https://github.com/rust-lang/rust-bindgen/pull/1489

--------------------------------------------------------------------------------

# 0.33.1 .. 0.46.0

https://github.com/rust-lang/rust-bindgen/compare/v0.32.2...v0.46.0

(Just a sneak peek, since a lot of stuff has changed :D)

## Added

* APIs to add lines to specific rust modules / C++ namespaces exist now.
  [#1307][]

[#1307]: https://github.com/rust-lang/rust-bindgen/issues/1307

## Removed

* The link options (`link`, `link_framework`, `link_static`) have been removed.
  They did nothing already, see [#104][]

[#104]: https://github.com/rust-lang/rust-bindgen/issues/104

## Changed

* Associated constants are used now for bitfield enums when available. [#1166][]

[#1166]: https://github.com/rust-lang/rust-bindgen/issues/1166

* New versions of a bunch of dependencies (syn / quote / etc.).

## Fixed

* Better target information from clang to properly generate types when
  cross-compiling [#1289][].

[#1289]: https://github.com/rust-lang/rust-bindgen/issues/1289

* Pointer constness was fixed in a bunch of cases when using `int const*` and
  such. [#1311][] [#1312][].

[#1311]: https://github.com/rust-lang/rust-bindgen/issues/1311
[#1312]: https://github.com/rust-lang/rust-bindgen/issues/1312

* Bitfields now work properly on big-endian machines. [#1340][]

[#1340]: https://github.com/rust-lang/rust-bindgen/issues/1340

* `wchar_t` layout works properly now. [#1345][]

[#1345]: https://github.com/rust-lang/rust-bindgen/issues/1345

* Functions can be blacklisted now. [#1364][]

[#1364]: https://github.com/rust-lang/rust-bindgen/issues/1364

* ... Lot's more!

--------------------------------------------------------------------------------

# 0.33.1

Released 2018/02/14

## Fixed

* Reverted the dependency update to `quote = "0.4"` and addition of the
  `proc_macro2` dependency. The `proc_macro2` crate depends on `rustc` internal
  libraries, which means that CLIs which use it must be run under `rustup`,
  which is not acceptable for `bindgen`. [#1248][]

[#1248]: https://github.com/rust-lang/rust-bindgen/issues/1248

--------------------------------------------------------------------------------

# 0.33.0

--------------------------------------------------------------------------------

# 0.32.2

Released 2018/01/22

## Fixed

* Avoid symbol generation for pure virtual functions. [#1197][]
* Handling of `_Complex _Float128`. [#1087][]
* Regression on code generation for variadic functions. [#1216][]
* Enum code generation generates conflicting repr hint warning. [#1224][]
* Constified code generation for enums with an explicit type of `bool`. [#1145][]
* Bindgen will now call `rustfmt` directly instead of via `rustup`. [#1184][]

[#1197]: https://github.com/rust-lang/rust-bindgen/issues/1197
[#1087]: https://github.com/rust-lang/rust-bindgen/issues/1087
[#1216]: https://github.com/rust-lang/rust-bindgen/issues/1216
[#1224]: https://github.com/rust-lang/rust-bindgen/issues/1224
[#1145]: https://github.com/rust-lang/rust-bindgen/issues/1145
[#1184]: https://github.com/rust-lang/rust-bindgen/issues/1184

# 0.32.1

Released 2017/12/18

## Fixed

* When translating C/C++ `enum`s into Rust `enum`s using `rustified_enum` /
  `--rustified-enum`, properly add `#[repr(C)]` to the emitted `enum`. [#1183][]

[#1183]: https://github.com/rust-lang/rust-bindgen/issues/1183

--------------------------------------------------------------------------------

# 0.32.0

Released 2017/12/08

## Added

* Added support for bit-field allocation units that are larger than 64 bits
  wide. Note that individual bit-fields within such units are still restricted
  to being no wider than 64 bits. [#1158][]

* We can now generate random C header files and test that `bindgen` can process
  them with the `quickcheck` crate. Initial support landed in [#1159][] with a
  few more additions in follow up pull requests.

## Changed

* The `bindgen::Builder::{constified_enum_module,{bitfield,rustified}_enum}`
  builder methods and their corresponding CLI flags now compare their argument
  to the C/C++ `enum`'s "canonical path", which includes leading namespaces,
  rather than its "canonical name", which does not. This is a breaking change
  that requires callers which target a namespaced C++ enum to call e.g.
  `bitfield_enum("<namespace>::<enum_name>")` rather than e.g.
  `bitfield_enum("<enum_name>")`. [#1162][]

* When a struct is packed to a smaller alignment that is still greater than one,
  `bindgen` cannot emit Rust bindings that match the input source. Before, it
  would emit `#[repr(packed)]` anyways, which packs to an alignment of one, but
  this can lead to misalignment and UB. Now, `bindgen` will detect these
  situations and convert the struct into an opaque blob of bytes with the proper
  alignment. We are eagerly awaiting support for `#[repr(packed(N))]` in
  Rust. [#1136][]

## Fixed

* There was a perfect storm of conditions that could cause `bindgen` not to emit
  any bindings if spawning `rustfmt` to format the bindings failed. This is now
  fixed. [#1112][]

* In some circumstances, `bindgen` would emit type parameters twice for
  references to template instantiations. This is now fixed. [#1113][]

* When a C/C++ struct had a field named with a Rust keyword, and `impl_debug`
  was enabled, the generated `impl Debug for ...` blocks could reference the
  field by the Rust keyword name, rather than the non-keyword field name we
  actually end up generating. This is now fixed. [#1123][]

* There was a regression in 0.31.0 where C++ template aliases to opaque types
  would sometimes not treat the aliased type as opaque. This is now
  fixed. [#1118][]

* There was a regression in 0.31.0 that could cause `bindgen` to panic when
  parsing nested template classes. This is now fixed. [#1127][]

* Unnamed bit-fields do not affect alignment of their struct or class in C/C++,
  however `bindgen` interpreted them as doing so, which could generate
  `#[repr(C)]` structs expecting to have an incorrect alignment. This is now
  fixed. [#1076][]

* When a zero-sized type was used in a bit-field, `bindgen` could
  divide-by-zero. This is now fixed. [#1137][]

* When a template parameter is used in a bit-field, `bindgen` would panic. This
  is now fixed. [#1140][]

* There was a regression in 0.31.0 where if `bindgen` was given a header file
  that did not exist, it would panic. This is now fixed, and it will instead
  properly report the error. [#1146][]

* In some cases, generated bit-field getters and setters could access memory
  beyond `self`. This is now fixed. [#954][]

[#1162]: https://github.com/rust-lang/rust-bindgen/issues/1162
[#1113]: https://github.com/rust-lang/rust-bindgen/issues/1113
[#1112]: https://github.com/rust-lang/rust-bindgen/issues/1112
[#1123]: https://github.com/rust-lang/rust-bindgen/issues/1123
[#1127]: https://github.com/rust-lang/rust-bindgen/issues/1127
[#1136]: https://github.com/rust-lang/rust-bindgen/issues/1136
[#1137]: https://github.com/rust-lang/rust-bindgen/issues/1137
[#1140]: https://github.com/rust-lang/rust-bindgen/issues/1140
[#1146]: https://github.com/rust-lang/rust-bindgen/issues/1146
[#1118]: https://github.com/rust-lang/rust-bindgen/issues/1118
[#1076]: https://github.com/rust-lang/rust-bindgen/issues/1076
[#1158]: https://github.com/rust-lang/rust-bindgen/issues/1158

--------------------------------------------------------------------------------

# 0.31.0

Released 2017/10/27

## Added

* 🎉 **A new `bindgen` reviewer: [@pepyakin](https://github.com/pepyakin)** 🎉
  You can ask @pepyakin to review all your future pull requests with `r?
  @pepyakin` from now on 😄

* Timers for seeing which phases `bindgen` is spending its time in. On the
  command line, use the `--time-phases` flag. From a builder, use the
  `bindgen::Builder::time_phases(true)` method. [#938][]

* You can now disable `#[derive(Copy)]` for all types with `--no-derive-copy`
  and `bindgen::Builder::derive_copy(false)`. [#948][]

* We now have an overview of `bindgen`'s code base and architecture for
  newcomers in `CONTRIBUTING.md`. [#988][]

* Derive `PartialOrd` with the `--with-derive-partialord` CLI flag or
  `bindgen::Builder::derive_partialord(true)` builder method. [#882][]

* Derive `Ord` with the `--with-derive-ord` CLI flag or
  `bindgen::Builder::derive_ord(true)` builder method. [#884][]

* When `PartialEq` cannot be derived because of an array larger than Rust's
  array-derive limit, `bindgen` can emit an `impl PartialEq for ...`
  block. Enable this behavior with the `--impl-partialeq` CLI flag or the
  `bindgen::Builder::impl_partialeq(true)` method. [#1012][]

* When deriving `PartialEq` for all types, you can now specify particular types
  that shouldn't `derive(PartialEq)` with the `--no-partialeq <regex>` CLI flag
  or `bindgen::Builder::no_partialeq("<regex>")` builder method. [#996][]

* Specify types that should not derive `Copy` with the `--no-copy <regex>` CLI
  flag or `bindgen::Builder::no_copy("<regex>")` builder method. This
  functionality was previously only available via comment annotations in the
  header sources. [#1099][]

* When deriving `Hash` for all types, you can now specify particular types that
  shouldn't `derive(Hash)` with the `--no-hash <regex>` CLI flag or
  `bindgen::Builder::no_hash("<regex>")` builder method. [#1105][]

* The `bindgen` users guide now has an [FAQ section][faq]! If you have any FAQ
  suggestions to put up there, please open a pull request. [#1020][]

* Added `csmith` fuzzing infrastructure. `csmith` generates random C and C++
  programs, we feed those into `bindgen` as headers to generate bindings to,
  then test that the generated bindings compile and that their layout tests
  pass. This infrastructure landed in
  [many small bits](https://github.com/rust-lang/rust-bindgen/issues?utf8=%E2%9C%93&q=label%3AA-csmith%20is%3Aclosed).

  We <3 folks who [help us find and fix issues via fuzzing][fuzzing]! *hint
  hint*

* Added experimental support for the `thiscall` ABI when targetting Rust
  nightly. [#1065][]

## Changed

* If the user does not explicitly pass a `--target` argument for `libclang`,
  `bindgen` will insert such an argument itself. See [#942][], [#947][], and
  [#953][] for details.

* C/C++ `enum`s are now translated into constants by default, rather than Rust
  `enum`s. The old behavior was a big footgun because `rustc` assumes that the
  only values of an `enum` are its variants, whereas a lot of C/C++ code uses
  random values as `enum`s. Put these two things and it leads to *undefined
  behavior*. Translating C/C++ `enum`s into Rust `enum`s is still available with
  the `--rustified-enum <regex>` CLI flag and
  `bindgen::Builder::rustified_enum("<regex>")` builder method. [#758][]

* Generated bindings are now pretty printed with `rustfmt` by default.
  Previously, this option existed, but was off by default because `syntex` did
  an OK job at pretty printing the bindings. Now that we are using `quote! {
  ... }` instead of `syntex`, we lost that pretty printing, and now rely on
  `rustfmt`. You can disable `rustfmt`ing with `--no-rustfmt-bindings` or
  `bindgen::Builder::rustfmt_bindings(false)`. See [#925][] and [#1022][] for
  details.

## Deprecated

* `bindgen::Builder::hide_type` is deprecated in favor of
  `bindgen::Builder::blacklist_type`. [#987][]

* `bindgen::Builder::whitelisted_type` is deprecated in favor of
  `bindgen::Builder::whitelist_type`. [#987][]

* `bindgen::Builder::whitelisted_function` is deprecated in favor of
  `bindgen::Builder::whitelist_function`. [#985][]

* `bindgen::Builder::whitelisted_var` is deprecated in favor of
  `bindgen::Builder::whitelist_var`. [#989][]

## Removed

* Removed the dependency on (unmaintained) `syntex`, and **build times are cut
  in half**!

  Before:

  ```
  $ cargo clean; cargo build
  <snip>
      Finished dev [unoptimized + debuginfo] target(s) in 98.75 secs
  ```

  After:

  ```
  $ cargo clean; cargo build
  <snip>
      Finished dev [unoptimized + debuginfo] target(s) in 46.26 secs
  ```

  [#925][]

* The `BindgenOptions` type is no longer public. It had been deprecated in
  previous releases. Use `bindgen::Builder` instead. [#1000][]

## Fixed

* Under certain conditions, a globally scoped `enum` could end up with bindings
  in the wrong namespace module. [#888][]

* Blacklisted types were incorrectly assumed to always be `Copy`able (and
  assumed to implement other traits as well). `bindgen` is now conservatively
  pessimistic about the traits that blacklisted types implement. [#944][]

* When bitfields have a ridiculously large number of bits (for example,
  `unsigned : 632;`) then `bindgen` was incorrectly deriving traits that
  couldn't be derived, resulting in errors when compiling the bindings, and was
  also generating `struct`s with an incorrect layout. Both issues have been
  fixed. [#982][]

* `_` is a valid identifier in some C++ contexts, but can't be referenced in
  Rust, as it is the "throwaway identifier" (a term I just made up, if you use
  it now, then you owe me money). `bindgen` will now translate `_` into `__` so
  that it can be used on the Rust side. [#1008][]

* Nested class definitions were sometimes being emitted in the wrong namespace
  module in the generated bindings. [#1048][]

* `bindgen` was mis-handling `union`s that contained bitfield members. This has
  been fixed. [#744][]

* Unsigned constants that were greater than `u32::MAX` were being mis-translated
  by `bindgen`. This is now fixed. [#1040][]

* When given a directory as an input file, or a file to which we don't have read
  permissions, then `bindgen` will print a more useful error message
  now. [#1029][]

* `bindgen` previously attempted to derive `Hash` for structures with
  flexibly-sized array members, but knowing how many elements exist in such
  arrays requires program-specific knowledge that `bindgen` cannot
  have. [#1094][]

[faq]: https://rust-lang.github.io/rust-bindgen/faq.html
[fuzzing]: https://github.com/rust-lang/rust-bindgen/blob/main/csmith-fuzzing/README.md

[#938]: https://github.com/rust-lang/rust-bindgen/issues/938
[#888]: https://github.com/rust-lang/rust-bindgen/issues/888
[#944]: https://github.com/rust-lang/rust-bindgen/issues/944
[#942]: https://github.com/rust-lang/rust-bindgen/issues/942
[#947]: https://github.com/rust-lang/rust-bindgen/issues/947
[#953]: https://github.com/rust-lang/rust-bindgen/issues/953
[#948]: https://github.com/rust-lang/rust-bindgen/issues/948
[#925]: https://github.com/rust-lang/rust-bindgen/issues/925
[#758]: https://github.com/rust-lang/rust-bindgen/issues/758
[#988]: https://github.com/rust-lang/rust-bindgen/issues/988
[#987]: https://github.com/rust-lang/rust-bindgen/issues/987
[#985]: https://github.com/rust-lang/rust-bindgen/issues/985
[#989]: https://github.com/rust-lang/rust-bindgen/issues/989
[#1000]: https://github.com/rust-lang/rust-bindgen/issues/1000
[#882]: https://github.com/rust-lang/rust-bindgen/issues/882
[#884]: https://github.com/rust-lang/rust-bindgen/issues/884
[#996]: https://github.com/rust-lang/rust-bindgen/issues/996
[#982]: https://github.com/rust-lang/rust-bindgen/issues/982
[#1008]: https://github.com/rust-lang/rust-bindgen/issues/1008
[#1022]: https://github.com/rust-lang/rust-bindgen/issues/1022
[#1048]: https://github.com/rust-lang/rust-bindgen/issues/1048
[#1012]: https://github.com/rust-lang/rust-bindgen/issues/1012
[#744]: https://github.com/rust-lang/rust-bindgen/issues/744
[#1065]: https://github.com/rust-lang/rust-bindgen/issues/1065
[#1040]: https://github.com/rust-lang/rust-bindgen/issues/1040
[#1029]: https://github.com/rust-lang/rust-bindgen/issues/1029
[#1094]: https://github.com/rust-lang/rust-bindgen/issues/1094
[#1099]: https://github.com/rust-lang/rust-bindgen/issues/1099
[#1105]: https://github.com/rust-lang/rust-bindgen/issues/1105

--------------------------------------------------------------------------------

# 0.30.0

Released 2017/08/28

## Added

* Explicit control over choosing which Rust version (specific stable versions or
  nightly Rust) to target. This defaults to the latest stable Rust
  version. [#832][]

```rust
bindgen::Builder::default()
    .rust_target(bindgen::RustTarget::Stable_1_19)
    // or `.rust_target(bindgen::RustTarget::Nightly)` to use unstable features
```

or

```
$ bindgen --rust-target 1.19
# or `--rust-target nightly` to use unstable features
```

* Started adding `derive(Copy)` for large arrays of `Copy` things, even when the
  array is too large to `derive(Clone)` because Rust doesn't implement `Clone`
  for arrays of length greater than 32. [#874][]

* `bindgen` can now determine which types are hashable and add `derive(Hash)` to
  those types that support it. This is disabled by default, but can be enabled
  via `bindgen::Builder::derive_hash` or `--with-derive-hash`. [#876][]

* `bindgen` can now generate `impl Debug for Blah` trait implementations for
  types that contain non-`Debug` types, and therefore cannot
  `derive(Debug)`. This behavior can be enabled with
  `bindgen::Builder::impl_debug` and `--impl-debug`. [#875][]

* `bindgen` can now invoke `rustfmt` on the generated bindings. The bindings
  have historically been fairly pretty printed, but sometimes this is not the
  case, especially with the new `impl Debug for Blah` feature. Have `bindgen`
  run `rustfmt` with `bindgen::Builder::rustfmt_bindings` and
  `--rustfmt-bindings`, and use non-default `rustfmt` configuration files with
  `bindgen::Builder::rustfmt_configuration_file` and
  `--rustfmt-configuration-file`. [#900][]

* `bindgen` can now determine which types can be compared with `==` and add
  `derive(PartialEq)` to those types that support it. This is disabled by
  default, but can be enabled via `bindgen::Builder::derive_partialeq` or
  `--with-derive-partialeq`. [#878][]

* Additionally, `bindgen` can also add `derive(Eq)` to those types which we
  determined we could `derive(PartialEq)` and do not transitively contain any
  floats. Enable this behavior with `bindgen::Builder::derive_eq` or
  `--with-derive-eq`. [#880][]

## Changed

* Started emitting Rust `union`s when targeting stable Rust >= 1.19, not just
  unstable nightly Rust. [#832][]

* Emitted layout `#[test]`s no longer contain internal IDs for template
  instantiations including pointers and arrays. This should make generated
  bindings more stable across updates to unrelated parts of the input
  headers. [#871][]

* Determining whether a type can derive `Copy` or not was ported from an ad-hoc
  algorithm to our fix-point framework. [#766][]

* Determining whether a type has a destructor or not was also ported from an
  ad-hoc algorithm to our fix-point framework. [#927][]

## Deprecated

* `bindgen::Builder::unstable_rust`/`--unstable-rust` is deprecated, in favor of
  targeting explicit Rust versions with
  `bindgen::Builder::rust_target`/`--rust-target` instead. [#832][]

## Fixed

* Fixed a regression in the `derive(Default)` analysis that resulted in some
  opaque types deriving `Default` when they shouldn't have. [#889][]

* Fixed a regression where template instantiation layout `#[test]`s were being
  generated with invalid Rust identifiers. [#906][]

[#832]: https://github.com/rust-lang/rust-bindgen/issues/832
[#871]: https://github.com/rust-lang/rust-bindgen/issues/871
[#874]: https://github.com/rust-lang/rust-bindgen/pull/874
[#889]: https://github.com/rust-lang/rust-bindgen/pull/874
[#766]: https://github.com/rust-lang/rust-bindgen/issues/766
[#876]: https://github.com/rust-lang/rust-bindgen/issues/876
[#875]: https://github.com/rust-lang/rust-bindgen/issues/875
[#906]: https://github.com/rust-lang/rust-bindgen/pull/906
[#900]: https://github.com/rust-lang/rust-bindgen/issues/900
[#878]: https://github.com/rust-lang/rust-bindgen/issues/878
[#880]: https://github.com/rust-lang/rust-bindgen/issues/880
[#927]: https://github.com/rust-lang/rust-bindgen/issues/927

--------------------------------------------------------------------------------

# 0.29.0

Released 2017/07/31

## Added

* ["Constified enum modules"](https://github.com/rust-lang/rust-bindgen/pull/741)
  translating C/C++ `enum`s into constants within a module for namespacing,
  rather than mangling the name of the generated constants.

  For example, it turns this:

  ```c++
  // bindgen-flags: --constified-enum-module PetKind

  enum PetKind {
      Doggo,
      Kitty,
      Hamster
  };

  struct Pet {
      PetKind kind;
      char* noise;
  };
  ```

  Into this:

  ```rust
  /* automatically generated by rust-bindgen */

  pub mod PetKind {
      pub type Type = ::std::os::raw::c_uint;
      pub const Doggo: Type = 0;
      pub const Kitty: Type = 1;
      pub const Hamster: Type = 2;
  }
  #[repr(C)]
  #[derive(Debug, Copy)]
  pub struct Pet {
      pub kind: PetKind::Type,
      pub noise: *mut ::std::os::raw::c_char,
  }
  ```

  The default translation strategy for `enum`s will generate constants with
  names like `PetKind_Hamster` instead.

  Use `bindgen::Builder::constified_enum_module` or `--constified-enum-module`.

* You can now
  [mark particular template instantiations as "opaque"](https://github.com/rust-lang/rust-bindgen/pull/773),
  so that `bindgen` emits a blob of bytes with the correct size and alignment
  rather than creating generic Rust types. This is useful as a workaround for
  when a template has a specialization for the given type arguments, which
  `bindgen` does not yet support. Previously, it was all of a templates'
  instantiations would be opaque or none of them would be. Use
  `bindgen::Builder::opaque_type("SomeTemplate<Foo, Bar>")` or `--opaque-type
  "SomeTemplate<Foo, Bar>"`.

* Added the ability to
  [preprocess and dump](https://github.com/rust-lang/rust-bindgen/pull/812)
  the input headers given to `bindgen` to a file. This should make creating
  reproducible, system independent, standalone test cases much easier! Bring on
  the new issues! Use `bindgen::Builder::dump_preprocessed_input` or
  `--dump-preprocessed-input`.

* We now use a fix-point analysis to determine whether any given type can derive
  `Debug`, or whether it has an explicit virtual table pointer. Previously we
  were using an ad-hoc algorithm that had at various times suffered from things
  like going into infinite loops when coming across cycles. Hopefully those
  kinds of bugs are a thing of the past!
  [#767](https://github.com/rust-lang/rust-bindgen/issues/767)
  [#765](https://github.com/rust-lang/rust-bindgen/issues/765)

## Changed

* The `bindgen` repository has moved under the `rust-lang-nursery` umbrella! The
  new repository URL is https://github.com/rust-lang-nursery/rust-bindgen 🎉

## Fixed

* No longer generating layout tests for template instantiations using type
  arguments that we didn't generate bindings for (which then caused compilation
  errors). [#679](https://github.com/rust-lang/rust-bindgen/issues/769)

* Fixed function name mangling when cross compiling bindings for
  iOS. [#776](https://github.com/rust-lang/rust-bindgen/pull/776)

* Don't include parent `inline namespace`s' names in types' names. Names of
  types from some STLs were showing up like `std___cxx11_basic_string` when they
  should have been
  `std_basic_string`. [#789](https://github.com/rust-lang/rust-bindgen/issues/789)

* Fixed a bug where we wouldn't generate type definitions for some types
  referenced by an opaque type's methods, causing compilation
  errors. [#807](https://github.com/rust-lang/rust-bindgen/issues/807)

* Fixed function name mangling issues for win32
  targets. [#819](https://github.com/rust-lang/rust-bindgen/issues/819)

* Fixed a bug where `bindgen` was generating a generic type alias that didn't
  use its type parameter, which is illegal Rust code and caused compilation
  errors. [#820](https://github.com/rust-lang/rust-bindgen/issues/820)

* The generated size, alignment, and field offset unit tests now have stable
  names rather than sometimes including an internal identifier which is
  inherently unstable. This was causing unnecessary diffs when folks were
  checking in new versions of bindings into their VCS.
  [#394](https://github.com/rust-lang/rust-bindgen/issues/394)

* Fixed a bug where we would try and `derive(Debug, Default)` on structs that
  had padding like `[u8; 33]`, which is larger than the largest array length for
  which Rust will derive traits. This would cause compilation errors when
  compiling the emitted bindings.
  [#648](https://github.com/rust-lang/rust-bindgen/issues/648)
