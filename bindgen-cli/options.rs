use bindgen::{
    builder, AliasVariation, Builder, CodegenConfig, EnumVariation,
    MacroTypeVariation, NonCopyUnionStyle, RustTarget,
    DEFAULT_ANON_FIELDS_PREFIX, RUST_TARGET_STRINGS,
};
use clap::{builder::PossibleValuesParser, Arg, ArgAction, Command};
use std::fs::File;
use std::io::{self, Error, ErrorKind};
use std::path::PathBuf;
use std::str::FromStr;

/// Construct a new [`Builder`](./struct.Builder.html) from command line flags.
pub fn builder_from_flags<I>(
    args: I,
) -> Result<(Builder, Box<dyn io::Write>, bool), io::Error>
where
    I: Iterator<Item = String>,
{
    let rust_target_help = format!(
        "Version of the Rust compiler to target. Valid options are: {:?}. Defaults to {:?}.",
        RUST_TARGET_STRINGS,
        String::from(RustTarget::default())
    );

    let matches = Command::new("bindgen")
        .about("Generates Rust bindings from C/C++ headers.")
        .disable_version_flag(true)
        .override_usage("bindgen [FLAGS] [OPTIONS] <header> -- <clang-args>...")
        .args(&[
            Arg::new("header")
                .help("C or C++ header file")
                .required_unless_present("V"),
            Arg::new("depfile")
                .long("depfile")
                .action(ArgAction::Set)
                .help("Path to write depfile to"),
            Arg::new("default-enum-style")
                .long("default-enum-style")
                .help("The default style of code used to generate enums.")
                .value_name("variant")
                .default_value("consts")
                .value_parser(PossibleValuesParser::new([
                    "consts",
                    "moduleconsts",
                    "bitfield",
                    "newtype",
                    "rust",
                    "rust_non_exhaustive",
                ]))
                .action(ArgAction::Set),
            Arg::new("bitfield-enum")
                .long("bitfield-enum")
                .help(
                    "Mark any enum whose name matches <regex> as a set of \
                     bitfield flags.",
                )
                .value_name("regex")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("newtype-enum")
                .long("newtype-enum")
                .help("Mark any enum whose name matches <regex> as a newtype.")
                .value_name("regex")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("newtype-global-enum")
                .long("newtype-global-enum")
                .help("Mark any enum whose name matches <regex> as a global newtype.")
                .value_name("regex")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("rustified-enum")
                .long("rustified-enum")
                .help("Mark any enum whose name matches <regex> as a Rust enum.")
                .value_name("regex")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("constified-enum")
                .long("constified-enum")
                .help(
                    "Mark any enum whose name matches <regex> as a series of \
                     constants.",
                )
                .value_name("regex")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("constified-enum-module")
                .long("constified-enum-module")
                .help(
                    "Mark any enum whose name matches <regex> as a module of \
                     constants.",
                )
                .value_name("regex")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("default-macro-constant-type")
                .long("default-macro-constant-type")
                .help("The default signed/unsigned type for C macro constants.")
                .value_name("variant")
                .default_value("unsigned")
                .value_parser(PossibleValuesParser::new(["signed", "unsigned"]))
                .action(ArgAction::Set),
            Arg::new("default-alias-style")
                .long("default-alias-style")
                .help("The default style of code used to generate typedefs.")
                .value_name("variant")
                .default_value("type_alias")
                .value_parser(PossibleValuesParser::new([
                    "type_alias",
                    "new_type",
                    "new_type_deref",
                ]))
                .action(ArgAction::Set),
            Arg::new("normal-alias")
                .long("normal-alias")
                .help(
                    "Mark any typedef alias whose name matches <regex> to use \
                     normal type aliasing.",
                )
                .value_name("regex")
                .action(ArgAction::Append)
                .number_of_values(1),
             Arg::new("new-type-alias")
                .long("new-type-alias")
                .help(
                    "Mark any typedef alias whose name matches <regex> to have \
                     a new type generated for it.",
                )
                .value_name("regex")
                .action(ArgAction::Append)
                .number_of_values(1),
             Arg::new("new-type-alias-deref")
                .long("new-type-alias-deref")
                .help(
                    "Mark any typedef alias whose name matches <regex> to have \
                     a new type with Deref and DerefMut to the inner type.",
                )
                .value_name("regex")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("default-non-copy-union-style")
                .long("default-non-copy-union-style")
                .help(
                    "The default style of code used to generate unions with \
                     non-Copy members. Note that ManuallyDrop was first \
                     stabilized in Rust 1.20.0.",
                )
                .value_name("style")
                .default_value("bindgen_wrapper")
                .value_parser(PossibleValuesParser::new([
                    "bindgen_wrapper",
                    "manually_drop",
                ]))
                .action(ArgAction::Set),
            Arg::new("bindgen-wrapper-union")
                .long("bindgen-wrapper-union")
                .help(
                    "Mark any union whose name matches <regex> and who has a \
                     non-Copy member to use a bindgen-generated wrapper for \
                     fields.",
                )
                .value_name("regex")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("manually-drop-union")
                .long("manually-drop-union")
                .help(
                    "Mark any union whose name matches <regex> and who has a \
                     non-Copy member to use ManuallyDrop (stabilized in Rust \
                     1.20.0) for fields.",
                )
                .value_name("regex")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("blocklist-type")
                .long("blocklist-type")
                .help("Mark <type> as hidden.")
                .value_name("type")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("blocklist-function")
                .long("blocklist-function")
                .help("Mark <function> as hidden.")
                .value_name("function")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("blocklist-item")
                .long("blocklist-item")
                .help("Mark <item> as hidden.")
                .value_name("item")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("blocklist-file")
                .long("blocklist-file")
                .help("Mark all contents of <path> as hidden.")
                .value_name("path")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("no-layout-tests")
                .long("no-layout-tests")
                .help("Avoid generating layout tests for any type.")
                .action(ArgAction::SetTrue),
            Arg::new("no-derive-copy")
                .long("no-derive-copy")
                .help("Avoid deriving Copy on any type.")
                .action(ArgAction::SetTrue),
            Arg::new("no-derive-debug")
                .long("no-derive-debug")
                .help("Avoid deriving Debug on any type.")
                .action(ArgAction::SetTrue),
            Arg::new("no-derive-default")
                .long("no-derive-default")
                .hide(true)
                .help("Avoid deriving Default on any type.")
                .action(ArgAction::SetTrue),
            Arg::new("impl-debug").long("impl-debug").help(
                "Create Debug implementation, if it can not be derived \
                 automatically.",
            )
            .action(ArgAction::SetTrue),
            Arg::new("impl-partialeq")
                .long("impl-partialeq")
                .help(
                    "Create PartialEq implementation, if it can not be derived \
                     automatically.",
                )
                .action(ArgAction::SetTrue),
            Arg::new("with-derive-default")
                .long("with-derive-default")
                .help("Derive Default on any type.")
                .action(ArgAction::SetTrue),
            Arg::new("with-derive-hash")
                .long("with-derive-hash")
                .help("Derive hash on any type.")
                .action(ArgAction::SetTrue),
            Arg::new("with-derive-partialeq")
                .long("with-derive-partialeq")
                .help("Derive partialeq on any type.")
                .action(ArgAction::SetTrue),
            Arg::new("with-derive-partialord")
                .long("with-derive-partialord")
                .help("Derive partialord on any type.")
                .action(ArgAction::SetTrue),
            Arg::new("with-derive-eq")
                .long("with-derive-eq")
                .help(
                    "Derive eq on any type. Enable this option also \
                     enables --with-derive-partialeq",
                )
                .action(ArgAction::SetTrue),
            Arg::new("with-derive-ord")
                .long("with-derive-ord")
                .help(
                    "Derive ord on any type. Enable this option also \
                     enables --with-derive-partialord",
                )
                .action(ArgAction::SetTrue),
            Arg::new("no-doc-comments")
                .long("no-doc-comments")
                .help(
                    "Avoid including doc comments in the output, see: \
                     https://github.com/rust-lang/rust-bindgen/issues/426",
                )
                .action(ArgAction::SetTrue),
            Arg::new("no-recursive-allowlist")
                .long("no-recursive-allowlist")
                .help(
                    "Disable allowlisting types recursively. This will cause \
                     bindgen to emit Rust code that won't compile! See the \
                     `bindgen::Builder::allowlist_recursively` method's \
                     documentation for details.",
                )
                .action(ArgAction::SetTrue),
            Arg::new("objc-extern-crate")
                .long("objc-extern-crate")
                .help("Use extern crate instead of use for objc.")
                .action(ArgAction::SetTrue),
            Arg::new("generate-block")
                .long("generate-block")
                .help("Generate block signatures instead of void pointers.")
                .action(ArgAction::SetTrue),
            Arg::new("block-extern-crate")
                .long("block-extern-crate")
                .help("Use extern crate instead of use for block.")
                .action(ArgAction::SetTrue),
            Arg::new("distrust-clang-mangling")
                .long("distrust-clang-mangling")
                .help("Do not trust the libclang-provided mangling")
                .action(ArgAction::SetTrue),
            Arg::new("builtins").long("builtins").help(
                "Output bindings for builtin definitions, e.g. \
                 __builtin_va_list.",
            )
            .action(ArgAction::SetTrue),
            Arg::new("ctypes-prefix")
                .long("ctypes-prefix")
                .help(
                    "Use the given prefix before raw types instead of \
                     ::std::os::raw.",
                )
                .value_name("prefix"),
            Arg::new("anon-fields-prefix")
                .long("anon-fields-prefix")
                .help("Use the given prefix for the anon fields.")
                .value_name("prefix")
                .default_value(DEFAULT_ANON_FIELDS_PREFIX),
            Arg::new("time-phases")
                .long("time-phases")
                .help("Time the different bindgen phases and print to stderr")
                .action(ArgAction::SetTrue),
            // All positional arguments after the end of options marker, `--`
            Arg::new("clang-args").last(true).action(ArgAction::Append),
            Arg::new("emit-clang-ast")
                .long("emit-clang-ast")
                .help("Output the Clang AST for debugging purposes.")
                .action(ArgAction::SetTrue),
            Arg::new("emit-ir")
                .long("emit-ir")
                .help("Output our internal IR for debugging purposes.")
                .action(ArgAction::SetTrue),
            Arg::new("emit-ir-graphviz")
                .long("emit-ir-graphviz")
                .help("Dump graphviz dot file.")
                .value_name("path"),
            Arg::new("enable-cxx-namespaces")
                .long("enable-cxx-namespaces")
                .help("Enable support for C++ namespaces.")
                .action(ArgAction::SetTrue),
            Arg::new("disable-name-namespacing")
                .long("disable-name-namespacing")
                .help(
                    "Disable namespacing via mangling, causing bindgen to \
                     generate names like \"Baz\" instead of \"foo_bar_Baz\" \
                     for an input name \"foo::bar::Baz\".",
                )
                .action(ArgAction::SetTrue),
            Arg::new("disable-nested-struct-naming")
                .long("disable-nested-struct-naming")
                .help(
                    "Disable nested struct naming, causing bindgen to generate \
                     names like \"bar\" instead of \"foo_bar\" for a nested \
                     definition \"struct foo { struct bar { } b; };\"."
                )
                .action(ArgAction::SetTrue),
            Arg::new("disable-untagged-union")
                .long("disable-untagged-union")
                .help(
                    "Disable support for native Rust unions.",
                )
                .action(ArgAction::SetTrue),
            Arg::new("disable-header-comment")
                .long("disable-header-comment")
                .help("Suppress insertion of bindgen's version identifier into generated bindings.")
                .action(ArgAction::SetTrue),
            Arg::new("ignore-functions")
                .long("ignore-functions")
                .help(
                    "Do not generate bindings for functions or methods. This \
                     is useful when you only care about struct layouts.",
                )
                .action(ArgAction::SetTrue),
            Arg::new("generate")
                .long("generate")
                .help(
                    "Generate only given items, split by commas. \
                     Valid values are \"functions\",\"types\", \"vars\", \
                     \"methods\", \"constructors\" and \"destructors\".",
                )
                .action(ArgAction::Set),
            Arg::new("ignore-methods")
                .long("ignore-methods")
                .help("Do not generate bindings for methods.")
                .action(ArgAction::SetTrue),
            Arg::new("no-convert-floats")
                .long("no-convert-floats")
                .help("Do not automatically convert floats to f32/f64.")
                .action(ArgAction::SetTrue),
            Arg::new("no-prepend-enum-name")
                .long("no-prepend-enum-name")
                .help("Do not prepend the enum name to constant or newtype variants.")
                .action(ArgAction::SetTrue),
            Arg::new("no-include-path-detection")
                .long("no-include-path-detection")
                .help("Do not try to detect default include paths")
                .action(ArgAction::SetTrue),
            Arg::new("fit-macro-constant-types")
                .long("fit-macro-constant-types")
                .help("Try to fit macro constants into types smaller than u32/i32")
                .action(ArgAction::SetTrue),
            Arg::new("opaque-type")
                .long("opaque-type")
                .help("Mark <type> as opaque.")
                .value_name("type")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Write Rust bindings to <output>.")
                .action(ArgAction::Set),
            Arg::new("raw-line")
                .long("raw-line")
                .help("Add a raw line of Rust code at the beginning of output.")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("module-raw-line")
                .long("module-raw-line")
                .help("Add a raw line of Rust code to a given module.")
                .action(ArgAction::Append)
                .number_of_values(2)
                .value_names(&["module-name", "raw-line"]),
            Arg::new("rust-target")
                .long("rust-target")
                .help(&rust_target_help)
                .action(ArgAction::Set),
            Arg::new("use-core")
                .long("use-core")
                .help("Use types from Rust core instead of std.")
                .action(ArgAction::SetTrue),
            Arg::new("conservative-inline-namespaces")
                .long("conservative-inline-namespaces")
                .help(
                    "Conservatively generate inline namespaces to avoid name \
                     conflicts.",
                )
                .action(ArgAction::SetTrue),
            Arg::new("use-msvc-mangling")
                .long("use-msvc-mangling")
                .help("MSVC C++ ABI mangling. DEPRECATED: Has no effect.")
                .action(ArgAction::SetTrue),
            Arg::new("allowlist-function")
                .long("allowlist-function")
                .help(
                    "Allowlist all the free-standing functions matching \
                     <regex>. Other non-allowlisted functions will not be \
                     generated.",
                )
                .value_name("regex")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("generate-inline-functions")
                .long("generate-inline-functions")
                .help("Generate inline functions.")
                .action(ArgAction::SetTrue),
            Arg::new("allowlist-type")
                .long("allowlist-type")
                .help(
                    "Only generate types matching <regex>. Other non-allowlisted types will \
                     not be generated.",
                )
                .value_name("regex")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("allowlist-var")
                .long("allowlist-var")
                .help(
                    "Allowlist all the free-standing variables matching \
                     <regex>. Other non-allowlisted variables will not be \
                     generated.",
                )
                .value_name("regex")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("allowlist-file")
                .alias("allowlist-file")
                .long("allowlist-file")
                .help("Allowlist all contents of <path>.")
                .value_name("path")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("verbose")
                .long("verbose")
                .help("Print verbose error messages.")
                .action(ArgAction::SetTrue),
            Arg::new("dump-preprocessed-input")
                .long("dump-preprocessed-input")
                .help(
                    "Preprocess and dump the input header files to disk. \
                     Useful when debugging bindgen, using C-Reduce, or when \
                     filing issues. The resulting file will be named \
                     something like `__bindgen.i` or `__bindgen.ii`.",
                )
                .action(ArgAction::SetTrue),
            Arg::new("no-record-matches")
                .long("no-record-matches")
                .help(
                    "Do not record matching items in the regex sets. \
                     This disables reporting of unused items.",
                )
                .action(ArgAction::SetTrue),
            Arg::new("size_t-is-usize")
                .long("size_t-is-usize")
                .help("Ignored - this is enabled by default.")
                .hide(true)
                .action(ArgAction::SetTrue),
            Arg::new("no-size_t-is-usize")
                .long("no-size_t-is-usize")
                .help("Do not bind size_t as usize (useful on platforms \
                       where those types are incompatible).")
                .action(ArgAction::SetTrue),
            Arg::new("no-rustfmt-bindings")
                .long("no-rustfmt-bindings")
                .help("Do not format the generated bindings with rustfmt.")
                .action(ArgAction::SetTrue),
            Arg::new("rustfmt-bindings")
                .long("rustfmt-bindings")
                .help(
                    "Format the generated bindings with rustfmt. DEPRECATED: \
                     --rustfmt-bindings is now enabled by default. Disable \
                     with --no-rustfmt-bindings.",
                ),
            Arg::new("rustfmt-configuration-file")
                .long("rustfmt-configuration-file")
                .help(
                    "The absolute path to the rustfmt configuration file. \
                     The configuration file will be used for formatting the bindings. \
                     This parameter is incompatible with --no-rustfmt-bindings.",
                )
                .value_name("path")
                .action(ArgAction::Set)
                .number_of_values(1),
            Arg::new("no-partialeq")
                .long("no-partialeq")
                .help("Avoid deriving PartialEq for types matching <regex>.")
                .value_name("regex")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("no-copy")
                .long("no-copy")
                .help("Avoid deriving Copy for types matching <regex>.")
                .value_name("regex")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("no-debug")
                .long("no-debug")
                .help("Avoid deriving Debug for types matching <regex>.")
                .value_name("regex")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("no-default")
                .long("no-default")
                .help("Avoid deriving/implement Default for types matching <regex>.")
                .value_name("regex")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("no-hash")
                .long("no-hash")
                .help("Avoid deriving Hash for types matching <regex>.")
                .value_name("regex")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("must-use-type")
                .long("must-use-type")
                .help("Add #[must_use] annotation to types matching <regex>.")
                .value_name("regex")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("enable-function-attribute-detection")
                .long("enable-function-attribute-detection")
                .help(
                    "Enables detecting unexposed attributes in functions (slow).
                       Used to generate #[must_use] annotations.",
                )
                .action(ArgAction::SetTrue),
            Arg::new("use-array-pointers-in-arguments")
                .long("use-array-pointers-in-arguments")
                .help("Use `*const [T; size]` instead of `*const T` for C arrays")
                .action(ArgAction::SetTrue),
            Arg::new("wasm-import-module-name")
                .long("wasm-import-module-name")
                .value_name("name")
                .help("The name to be used in a #[link(wasm_import_module = ...)] statement"),
            Arg::new("dynamic-loading")
                .long("dynamic-loading")
                .action(ArgAction::Set)
                .help("Use dynamic loading mode with the given library name."),
            Arg::new("dynamic-link-require-all")
                .long("dynamic-link-require-all")
                .help("Require successful linkage to all functions in the library.")
                .action(ArgAction::SetTrue),
            Arg::new("respect-cxx-access-specs")
                .long("respect-cxx-access-specs")
                .help("Makes generated bindings `pub` only for items if the items are publically accessible in C++.")
                .action(ArgAction::SetTrue),
            Arg::new("translate-enum-integer-types")
                .long("translate-enum-integer-types")
                .help("Always translate enum integer types to native Rust integer types.")
                .action(ArgAction::SetTrue),
            Arg::new("c-naming")
                .long("c-naming")
                .help("Generate types with C style naming.")
                .action(ArgAction::SetTrue),
            Arg::new("explicit-padding")
                .long("explicit-padding")
                .help("Always output explicit padding fields.")
                .action(ArgAction::SetTrue),
            Arg::new("vtable-generation")
                .long("vtable-generation")
                .help("Enables generation of vtable functions.")
                .action(ArgAction::SetTrue),
            Arg::new("sort-semantically")
                .long("sort-semantically")
                .help("Enables sorting of code generation in a predefined manner.")
                .action(ArgAction::SetTrue),
            Arg::new("merge-extern-blocks")
                .long("merge-extern-blocks")
                .help("Deduplicates extern blocks.")
                .action(ArgAction::SetTrue),
            Arg::new("override-abi")
                .long("override-abi")
                .help("Overrides the ABI of functions matching <regex>. The <override> value must be of the shape <regex>=<abi> where <abi> can be one of C, stdcall, fastcall, thiscall, aapcs, win64 or C-unwind.")
                .value_name("override")
                .action(ArgAction::Append)
                .number_of_values(1),
            Arg::new("wrap-unsafe-ops")
                .long("wrap-unsafe-ops")
                .help("Wrap unsafe operations in unsafe blocks.")
                .action(ArgAction::SetTrue),
            Arg::new("V")
                .long("version")
                .help("Prints the version, and exits")
                .action(ArgAction::SetTrue),
        ]) // .args()
        .get_matches_from(args);

    let verbose = matches.get_flag("verbose");
    if matches.get_flag("V") {
        println!(
            "bindgen {}",
            option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")
        );
        if verbose {
            println!("Clang: {}", bindgen::clang_version().full);
        }
        std::process::exit(0);
    }

    let mut builder = builder();

    if let Some(header) = matches.get_one::<String>("header") {
        builder = builder.header(header);
    } else {
        return Err(Error::new(ErrorKind::Other, "Header not found"));
    }

    if let Some(rust_target) = matches.get_one::<String>("rust-target") {
        builder = builder.rust_target(RustTarget::from_str(rust_target)?);
    }

    if let Some(variant) = matches.get_one::<String>("default-enum-style") {
        builder = builder.default_enum_style(EnumVariation::from_str(variant)?)
    }

    if let Some(bitfields) = matches.get_many::<String>("bitfield-enum") {
        for regex in bitfields {
            builder = builder.bitfield_enum(regex);
        }
    }

    if let Some(newtypes) = matches.get_many::<String>("newtype-enum") {
        for regex in newtypes {
            builder = builder.newtype_enum(regex);
        }
    }

    if let Some(newtypes) = matches.get_many::<String>("newtype-global-enum") {
        for regex in newtypes {
            builder = builder.newtype_global_enum(regex);
        }
    }

    if let Some(rustifieds) = matches.get_many::<String>("rustified-enum") {
        for regex in rustifieds {
            builder = builder.rustified_enum(regex);
        }
    }

    if let Some(const_enums) = matches.get_many::<String>("constified-enum") {
        for regex in const_enums {
            builder = builder.constified_enum(regex);
        }
    }

    if let Some(constified_mods) =
        matches.get_many::<String>("constified-enum-module")
    {
        for regex in constified_mods {
            builder = builder.constified_enum_module(regex);
        }
    }

    if let Some(variant) =
        matches.get_one::<String>("default-macro-constant-type")
    {
        builder = builder
            .default_macro_constant_type(MacroTypeVariation::from_str(variant)?)
    }

    if let Some(variant) = matches.get_one::<String>("default-alias-style") {
        builder =
            builder.default_alias_style(AliasVariation::from_str(variant)?);
    }

    if let Some(type_alias) = matches.get_many::<String>("normal-alias") {
        for regex in type_alias {
            builder = builder.type_alias(regex);
        }
    }

    if let Some(new_type) = matches.get_many::<String>("new-type-alias") {
        for regex in new_type {
            builder = builder.new_type_alias(regex);
        }
    }

    if let Some(new_type_deref) =
        matches.get_many::<String>("new-type-alias-deref")
    {
        for regex in new_type_deref {
            builder = builder.new_type_alias_deref(regex);
        }
    }

    if let Some(variant) =
        matches.get_one::<String>("default-non-copy-union-style")
    {
        builder = builder.default_non_copy_union_style(
            NonCopyUnionStyle::from_str(variant)?,
        );
    }

    if let Some(bindgen_wrapper_union) =
        matches.get_many::<String>("bindgen-wrapper-union")
    {
        for regex in bindgen_wrapper_union {
            builder = builder.bindgen_wrapper_union(regex);
        }
    }

    if let Some(manually_drop_union) =
        matches.get_many::<String>("manually-drop-union")
    {
        for regex in manually_drop_union {
            builder = builder.manually_drop_union(regex);
        }
    }

    if let Some(hidden_types) = matches.get_many::<String>("blocklist-type") {
        for ty in hidden_types {
            builder = builder.blocklist_type(ty);
        }
    }

    if let Some(hidden_functions) =
        matches.get_many::<String>("blocklist-function")
    {
        for fun in hidden_functions {
            builder = builder.blocklist_function(fun);
        }
    }

    if let Some(hidden_identifiers) =
        matches.get_many::<String>("blocklist-item")
    {
        for id in hidden_identifiers {
            builder = builder.blocklist_item(id);
        }
    }

    if let Some(hidden_files) = matches.get_many::<String>("blocklist-file") {
        for file in hidden_files {
            builder = builder.blocklist_file(file);
        }
    }

    if matches.get_flag("builtins") {
        builder = builder.emit_builtins();
    }

    if matches.get_flag("no-layout-tests") {
        builder = builder.layout_tests(false);
    }

    if matches.get_flag("no-derive-copy") {
        builder = builder.derive_copy(false);
    }

    if matches.get_flag("no-derive-debug") {
        builder = builder.derive_debug(false);
    }

    if matches.get_flag("impl-debug") {
        builder = builder.impl_debug(true);
    }

    if matches.get_flag("impl-partialeq") {
        builder = builder.impl_partialeq(true);
    }

    if matches.get_flag("with-derive-default") {
        builder = builder.derive_default(true);
    }

    if matches.get_flag("with-derive-hash") {
        builder = builder.derive_hash(true);
    }

    if matches.get_flag("with-derive-partialeq") {
        builder = builder.derive_partialeq(true);
    }

    if matches.get_flag("with-derive-partialord") {
        builder = builder.derive_partialord(true);
    }

    if matches.get_flag("with-derive-eq") {
        builder = builder.derive_eq(true);
    }

    if matches.get_flag("with-derive-ord") {
        builder = builder.derive_ord(true);
    }

    if matches.get_flag("no-derive-default") {
        builder = builder.derive_default(false);
    }

    if matches.get_flag("no-prepend-enum-name") {
        builder = builder.prepend_enum_name(false);
    }

    if matches.get_flag("no-include-path-detection") {
        builder = builder.detect_include_paths(false);
    }

    if matches.get_flag("fit-macro-constant-types") {
        builder = builder.fit_macro_constants(true);
    }

    if matches.get_flag("time-phases") {
        builder = builder.time_phases(true);
    }

    if matches.get_flag("use-array-pointers-in-arguments") {
        builder = builder.array_pointers_in_arguments(true);
    }

    if let Some(wasm_import_name) =
        matches.get_one::<String>("wasm-import-module-name")
    {
        builder = builder.wasm_import_module_name(wasm_import_name);
    }

    if let Some(prefix) = matches.get_one::<String>("ctypes-prefix") {
        builder = builder.ctypes_prefix(prefix);
    }

    if let Some(prefix) = matches.get_one::<String>("anon-fields-prefix") {
        builder = builder.anon_fields_prefix(prefix);
    }

    if let Some(what_to_generate) = matches.get_one::<String>("generate") {
        let mut config = CodegenConfig::empty();
        for what in what_to_generate.split(',') {
            match what {
                "functions" => config.insert(CodegenConfig::FUNCTIONS),
                "types" => config.insert(CodegenConfig::TYPES),
                "vars" => config.insert(CodegenConfig::VARS),
                "methods" => config.insert(CodegenConfig::METHODS),
                "constructors" => config.insert(CodegenConfig::CONSTRUCTORS),
                "destructors" => config.insert(CodegenConfig::DESTRUCTORS),
                otherwise => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("Unknown generate item: {}", otherwise),
                    ));
                }
            }
        }
        builder = builder.with_codegen_config(config);
    }

    if matches.get_flag("emit-clang-ast") {
        builder = builder.emit_clang_ast();
    }

    if matches.get_flag("emit-ir") {
        builder = builder.emit_ir();
    }

    if let Some(path) = matches.get_one::<String>("emit-ir-graphviz") {
        builder = builder.emit_ir_graphviz(path);
    }

    if matches.get_flag("enable-cxx-namespaces") {
        builder = builder.enable_cxx_namespaces();
    }

    if matches.get_flag("enable-function-attribute-detection") {
        builder = builder.enable_function_attribute_detection();
    }

    if matches.get_flag("disable-name-namespacing") {
        builder = builder.disable_name_namespacing();
    }

    if matches.get_flag("disable-nested-struct-naming") {
        builder = builder.disable_nested_struct_naming();
    }

    if matches.get_flag("disable-untagged-union") {
        builder = builder.disable_untagged_union();
    }

    if matches.get_flag("disable-header-comment") {
        builder = builder.disable_header_comment();
    }

    if matches.get_flag("ignore-functions") {
        builder = builder.ignore_functions();
    }

    if matches.get_flag("ignore-methods") {
        builder = builder.ignore_methods();
    }

    if matches.get_flag("no-convert-floats") {
        builder = builder.no_convert_floats();
    }

    if matches.get_flag("no-doc-comments") {
        builder = builder.generate_comments(false);
    }

    if matches.get_flag("no-recursive-allowlist") {
        builder = builder.allowlist_recursively(false);
    }

    if matches.get_flag("objc-extern-crate") {
        builder = builder.objc_extern_crate(true);
    }

    if matches.get_flag("generate-block") {
        builder = builder.generate_block(true);
    }

    if matches.get_flag("block-extern-crate") {
        builder = builder.block_extern_crate(true);
    }

    if let Some(opaque_types) = matches.get_many::<String>("opaque-type") {
        for ty in opaque_types {
            builder = builder.opaque_type(ty);
        }
    }

    if let Some(lines) = matches.get_many::<String>("raw-line") {
        for line in lines {
            builder = builder.raw_line(line);
        }
    }

    if let Some(mut values) = matches.get_many::<String>("module-raw-line") {
        while let Some(module) = values.next() {
            let line = values.next().unwrap();
            builder = builder.module_raw_line(module, line);
        }
    }

    if matches.get_flag("use-core") {
        builder = builder.use_core();
    }

    if matches.get_flag("distrust-clang-mangling") {
        builder = builder.trust_clang_mangling(false);
    }

    if matches.get_flag("conservative-inline-namespaces") {
        builder = builder.conservative_inline_namespaces();
    }

    if matches.get_flag("generate-inline-functions") {
        builder = builder.generate_inline_functions(true);
    }

    if let Some(allowlist) = matches.get_many::<String>("allowlist-function") {
        for regex in allowlist {
            builder = builder.allowlist_function(regex);
        }
    }

    if let Some(allowlist) = matches.get_many::<String>("allowlist-type") {
        for regex in allowlist {
            builder = builder.allowlist_type(regex);
        }
    }

    if let Some(allowlist) = matches.get_many::<String>("allowlist-var") {
        for regex in allowlist {
            builder = builder.allowlist_var(regex);
        }
    }

    if let Some(hidden_files) = matches.get_many::<String>("allowlist-file") {
        for file in hidden_files {
            builder = builder.allowlist_file(file);
        }
    }

    if let Some(args) = matches.get_many::<String>("clang-args") {
        for arg in args {
            builder = builder.clang_arg(arg);
        }
    }

    let output = if let Some(path) = matches.get_one::<String>("output") {
        let file = File::create(path)?;
        if let Some(depfile) = matches.get_one::<String>("depfile") {
            builder = builder.depfile(path, depfile);
        }
        Box::new(io::BufWriter::new(file)) as Box<dyn io::Write>
    } else {
        if let Some(depfile) = matches.get_one::<String>("depfile") {
            builder = builder.depfile("-", depfile);
        }
        Box::new(io::BufWriter::new(io::stdout())) as Box<dyn io::Write>
    };

    if matches.get_flag("dump-preprocessed-input") {
        builder.dump_preprocessed_input()?;
    }

    if matches.get_flag("no-record-matches") {
        builder = builder.record_matches(false);
    }

    if matches.get_flag("no-size_t-is-usize") {
        builder = builder.size_t_is_usize(false);
    }

    let no_rustfmt_bindings = matches.get_flag("no-rustfmt-bindings");
    if no_rustfmt_bindings {
        builder = builder.rustfmt_bindings(false);
    }

    if let Some(path_str) =
        matches.get_one::<String>("rustfmt-configuration-file")
    {
        let path = PathBuf::from(path_str);

        if no_rustfmt_bindings {
            return Err(Error::new(
                ErrorKind::Other,
                "Cannot supply both --rustfmt-configuration-file and --no-rustfmt-bindings",
            ));
        }

        if !path.is_absolute() {
            return Err(Error::new(
                ErrorKind::Other,
                "--rustfmt-configuration--file needs to be an absolute path!",
            ));
        }

        if path.to_str().is_none() {
            return Err(Error::new(
                ErrorKind::Other,
                "--rustfmt-configuration-file contains non-valid UTF8 characters.",
            ));
        }

        builder = builder.rustfmt_configuration_file(Some(path));
    }

    if let Some(no_partialeq) = matches.get_many::<String>("no-partialeq") {
        for regex in no_partialeq {
            builder = builder.no_partialeq(regex);
        }
    }

    if let Some(no_copy) = matches.get_many::<String>("no-copy") {
        for regex in no_copy {
            builder = builder.no_copy(regex);
        }
    }

    if let Some(no_debug) = matches.get_many::<String>("no-debug") {
        for regex in no_debug {
            builder = builder.no_debug(regex);
        }
    }

    if let Some(no_default) = matches.get_many::<String>("no-default") {
        for regex in no_default {
            builder = builder.no_default(regex);
        }
    }

    if let Some(no_hash) = matches.get_many::<String>("no-hash") {
        for regex in no_hash {
            builder = builder.no_hash(regex);
        }
    }

    if let Some(must_use_type) = matches.get_many::<String>("must-use-type") {
        for regex in must_use_type {
            builder = builder.must_use_type(regex);
        }
    }

    if let Some(dynamic_library_name) =
        matches.get_one::<String>("dynamic-loading")
    {
        builder = builder.dynamic_library_name(dynamic_library_name);
    }

    if matches.get_flag("dynamic-link-require-all") {
        builder = builder.dynamic_link_require_all(true);
    }

    if matches.get_flag("respect-cxx-access-specs") {
        builder = builder.respect_cxx_access_specs(true);
    }

    if matches.get_flag("translate-enum-integer-types") {
        builder = builder.translate_enum_integer_types(true);
    }

    if matches.get_flag("c-naming") {
        builder = builder.c_naming(true);
    }

    if matches.get_flag("explicit-padding") {
        builder = builder.explicit_padding(true);
    }

    if matches.get_flag("vtable-generation") {
        builder = builder.vtable_generation(true);
    }

    if matches.get_flag("sort-semantically") {
        builder = builder.sort_semantically(true);
    }

    if matches.get_flag("merge-extern-blocks") {
        builder = builder.merge_extern_blocks(true);
    }

    if let Some(abi_overrides) = matches.get_many::<String>("override-abi") {
        for abi_override in abi_overrides {
            let (regex, abi_str) = abi_override
                .rsplit_once('=')
                .expect("Invalid ABI override: Missing `=`");
            let abi = abi_str
                .parse()
                .unwrap_or_else(|err| panic!("Invalid ABI override: {}", err));
            builder = builder.override_abi(abi, regex);
        }
    }

    if matches.get_flag("wrap-unsafe-ops") {
        builder = builder.wrap_unsafe_ops(true);
    }

    Ok((builder, output, verbose))
}
