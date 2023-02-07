use bindgen::callbacks::TypeKind;
use bindgen::{
    builder, AliasVariation, Builder, CodegenConfig, EnumVariation,
    MacroTypeVariation, NonCopyUnionStyle, RegexSet, RustTarget,
    DEFAULT_ANON_FIELDS_PREFIX, RUST_TARGET_STRINGS,
};
use clap::Parser;
use std::fs::File;
use std::io::{self, Error, ErrorKind};
use std::path::PathBuf;

fn rust_target_help() -> String {
    format!(
        "Version of the Rust compiler to target. Valid options are: {:?}. Defaults to {:?}.",
        RUST_TARGET_STRINGS,
        String::from(RustTarget::default())
    )
}

fn parse_codegen_config(what_to_generate: &str) -> io::Result<CodegenConfig> {
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

    Ok(config)
}

#[derive(Parser, Debug)]
#[clap(
    about = "Generates Rust bindings from C/C++ headers.",
    override_usage = "bindgen [FLAGS] [OPTIONS] [HEADER] -- [CLANG_ARGS]...",
    trailing_var_arg = true
)]
struct BindgenCommand {
    /// C or C++ header file.
    header: Option<String>,
    /// Path to write depfile to.
    #[arg(long)]
    depfile: Option<String>,
    /// The default style of code used to generate enums.
    #[arg(long, value_name = "VARIANT")]
    default_enum_style: Option<EnumVariation>,
    /// Mark any enum whose name matches <REGEX> as a set of bitfield flags.
    #[arg(long, value_name = "REGEX")]
    bitfield_enum: Vec<String>,
    /// Mark any enum whose name matches <REGEX> as a newtype.
    #[arg(long, value_name = "REGEX")]
    newtype_enum: Vec<String>,
    /// Mark any enum whose name matches <REGEX> as a global newtype.
    #[arg(long, value_name = "REGEX")]
    newtype_global_enum: Vec<String>,
    /// Mark any enum whose name matches <REGEX> as a Rust enum.
    #[arg(long, value_name = "REGEX")]
    rustified_enum: Vec<String>,
    /// Mark any enum whose name matches <REGEX> as a series of constants.
    #[arg(long, value_name = "REGEX")]
    constified_enum: Vec<String>,
    /// Mark any enum whose name matches <regex> as a module of constants.
    #[arg(long, value_name = "REGEX")]
    constified_enum_module: Vec<String>,
    /// The default signed/unsigned type for C macro constants.
    #[arg(long, value_name = "VARIANT")]
    default_macro_constant_type: Option<MacroTypeVariation>,
    /// The default style of code used to generate typedefs.
    #[arg(long, value_name = "VARIANT")]
    default_alias_style: Option<AliasVariation>,
    /// Mark any typedef alias whose name matches <REGEX> to use normal type aliasing.
    #[arg(long, value_name = "REGEX")]
    normal_alias: Vec<String>,
    /// Mark any typedef alias whose name matches <REGEX> to have a new type generated for it.
    #[arg(long, value_name = "REGEX")]
    new_type_alias: Vec<String>,
    /// Mark any typedef alias whose name matches <REGEX> to have a new type with Deref and DerefMut to the inner type.
    #[arg(long, value_name = "REGEX")]
    new_type_alias_deref: Vec<String>,
    /// The default style of code used to generate unions with non-Copy members. Note that ManuallyDrop was first stabilized in Rust 1.20.0.
    #[arg(long, value_name = "STYLE")]
    default_non_copy_union_style: Option<NonCopyUnionStyle>,
    /// Mark any union whose name matches <REGEX> and who has a non-Copy member to use a bindgen-generated wrapper for fields.
    #[arg(long, value_name = "REGEX")]
    bindgen_wrapper_union: Vec<String>,
    /// Mark any union whose name matches <REGEX> and who has a non-Copy member to use ManuallyDrop (stabilized in Rust 1.20.0) for fields.
    #[arg(long, value_name = "REGEX")]
    manually_drop_union: Vec<String>,
    /// Mark <TYPE> as hidden.
    #[arg(long, value_name = "TYPE")]
    blocklist_type: Vec<String>,
    /// Mark <FUNCTION> as hidden.
    #[arg(long, value_name = "FUNCTION")]
    blocklist_function: Vec<String>,
    /// Mark <ITEM> as hidden.
    #[arg(long, value_name = "ITEM")]
    blocklist_item: Vec<String>,
    /// Mark <FILE> as hidden.
    #[arg(long, value_name = "FILE")]
    blocklist_file: Vec<String>,
    /// Avoid generating layout tests for any type.
    #[arg(long)]
    no_layout_tests: bool,
    /// Avoid deriving Copy on any type.
    #[arg(long)]
    no_derive_copy: bool,
    /// Avoid deriving Debug on any type.
    #[arg(long)]
    no_derive_debug: bool,
    /// Avoid deriving Default on any type.
    #[arg(long, hide = true)]
    no_derive_default: bool,
    /// Create a Debug implementation if it cannot be derived automatically.
    #[arg(long)]
    impl_debug: bool,
    /// Create a PartialEq implementation if it cannot be derived automatically.
    #[arg(long)]
    impl_partialeq: bool,
    /// Derive Default on any type.
    #[arg(long)]
    with_derive_default: bool,
    /// Derive Hash on any type.docstring
    #[arg(long)]
    with_derive_hash: bool,
    /// Derive PartialEq on any type.
    #[arg(long)]
    with_derive_partialeq: bool,
    /// Derive PartialOrd on any type.
    #[arg(long)]
    with_derive_partialord: bool,
    /// Derive Eq on any type.
    #[arg(long)]
    with_derive_eq: bool,
    /// Derive Ord on any type.
    #[arg(long)]
    with_derive_ord: bool,
    /// Avoid including doc comments in the output, see: https://github.com/rust-lang/rust-bindgen/issues/426
    #[arg(long)]
    no_doc_comments: bool,
    /// Disable allowlisting types recursively. This will cause bindgen to emit Rust code that won't compile! See the `bindgen::Builder::allowlist_recursively` method's documentation for details.
    #[arg(long)]
    no_recursive_allowlist: bool,
    /// Use extern crate instead of use for objc.
    #[arg(long)]
    objc_extern_crate: bool,
    /// Generate block signatures instead of void pointers.
    #[arg(long)]
    generate_block: bool,
    /// Use extern crate instead of use for block.
    #[arg(long)]
    block_extern_crate: bool,
    /// Do not trust the libclang-provided mangling
    #[arg(long)]
    distrust_clang_mangling: bool,
    /// Output bindings for builtin definitions, e.g. __builtin_va_list.
    #[arg(long)]
    builtins: bool,
    /// Use the given prefix before raw types instead of ::std::os::raw.
    #[arg(long, value_name = "PREFIX")]
    ctypes_prefix: Option<String>,
    /// Use the given prefix for anonymous fields.
    #[arg(long, default_value = DEFAULT_ANON_FIELDS_PREFIX, value_name = "PREFIX")]
    anon_fields_prefix: String,
    /// Time the different bindgen phases and print to stderr
    #[arg(long)]
    time_phases: bool,
    /// Output the Clang AST for debugging purposes.
    #[arg(long)]
    emit_clang_ast: bool,
    /// Output our internal IR for debugging purposes.
    #[arg(long)]
    emit_ir: bool,
    /// Dump graphviz dot file.
    #[arg(long, value_name = "PATH")]
    emit_ir_graphviz: Option<String>,
    /// Enable support for C++ namespaces.
    #[arg(long)]
    enable_cxx_namespaces: bool,
    /// Disable namespacing via mangling, causing bindgen to generate names like `Baz` instead of `foo_bar_Baz` for an input name `foo::bar::Baz`.
    #[arg(long)]
    disable_name_namespacing: bool,
    /// Disable nested struct naming, causing bindgen to generate names like `bar` instead of `foo_bar` for a nested definition `struct foo { struct bar { } b; };`.
    #[arg(long)]
    disable_nested_struct_naming: bool,
    /// Disable support for native Rust unions.
    #[arg(long)]
    disable_untagged_union: bool,
    /// Suppress insertion of bindgen's version identifier into generated bindings.
    #[arg(long)]
    disable_header_comment: bool,
    /// Do not generate bindings for functions or methods. This is useful when you only care about struct layouts.docstring
    #[arg(long)]
    ignore_functions: bool,
    /// Generate only given items, split by commas. Valid values are `functions`,`types`, `vars`, `methods`, `constructors` and `destructors`.
    #[arg(long, value_parser = parse_codegen_config)]
    generate: Option<CodegenConfig>,
    /// Do not generate bindings for methods.
    #[arg(long)]
    ignore_methods: bool,
    /// Do not automatically convert floats to f32/f64.
    #[arg(long)]
    no_convert_floats: bool,
    /// Do not prepend the enum name to constant or newtype variants.
    #[arg(long)]
    no_prepend_enum_name: bool,
    /// Do not try to detect default include paths
    #[arg(long)]
    no_include_path_detection: bool,
    /// Try to fit macro constants into types smaller than u32/i32
    #[arg(long)]
    fit_macro_constant_types: bool,
    /// Mark <TYPE> as opaque.
    #[arg(long, value_name = "TYPE")]
    opaque_type: Vec<String>,
    ///  Write Rust bindings to <OUTPUT>.
    #[arg(long, short, value_name = "OUTPUT")]
    output: Option<String>,
    /// Add a raw line of Rust code at the beginning of output.
    #[arg(long)]
    raw_line: Vec<String>,
    /// Add a raw line of Rust code to a given module.
    #[arg(long, number_of_values = 2, value_names = ["MODULE-NAME", "RAW-LINE"])]
    module_raw_line: Vec<String>,
    #[arg(long, help = rust_target_help())]
    rust_target: Option<RustTarget>,
    /// Use types from Rust core instead of std.
    #[arg(long)]
    use_core: bool,
    /// Conservatively generate inline namespaces to avoid name conflicts.
    #[arg(long)]
    conservative_inline_namespaces: bool,
    /// MSVC C++ ABI mangling. DEPRECATED: Has no effect.
    #[arg(long)]
    use_msvc_mangling: bool,
    /// Allowlist all the free-standing functions matching <REGEX>. Other non-allowlisted functions will not be generated.
    #[arg(long, value_name = "REGEX")]
    allowlist_function: Vec<String>,
    /// Generate inline functions.
    #[arg(long)]
    generate_inline_functions: bool,
    /// Only generate types matching <REGEX>. Other non-allowlisted types will not be generated.
    #[arg(long, value_name = "REGEX")]
    allowlist_type: Vec<String>,
    /// Allowlist all the free-standing variables matching <REGEX>. Other non-allowlisted variables will not be generated.
    #[arg(long, value_name = "REGEX")]
    allowlist_var: Vec<String>,
    /// Allowlist all contents of <PATH>.
    #[arg(long, value_name = "PATH")]
    allowlist_file: Vec<String>,
    /// Print verbose error messages.
    #[arg(long)]
    verbose: bool,
    /// Preprocess and dump the input header files to disk. Useful when debugging bindgen, using C-Reduce, or when filing issues. The resulting file will be named something like `__bindgen.i` or `__bindgen.ii`.
    #[arg(long)]
    dump_preprocessed_input: bool,
    /// Do not record matching items in the regex sets. This disables reporting of unused items.
    #[arg(long)]
    no_record_matches: bool,
    /// Ignored - this is enabled by default.
    #[arg(long = "size_t-is-usize")]
    size_t_is_usize: bool,
    /// Do not bind size_t as usize (useful on platforms where those types are incompatible).
    #[arg(long = "no-size_t-is-usize")]
    no_size_t_is_usize: bool,
    /// Do not format the generated bindings with rustfmt.
    #[arg(long)]
    no_rustfmt_bindings: bool,
    /// Format the generated bindings with rustfmt. DEPRECATED: --rustfmt-bindings is now enabled by default. Disable with --no-rustfmt-bindings.
    #[arg(long)]
    rustfmt_bindings: bool,
    /// The absolute path to the rustfmt configuration file. The configuration file will be used for formatting the bindings. This parameter is incompatible with --no-rustfmt-bindings.
    #[arg(long, value_name = "PATH")]
    rustfmt_configuration_file: Option<String>,
    /// Avoid deriving PartialEq for types matching <REGEX>.
    #[arg(long, value_name = "REGEX")]
    no_partialeq: Vec<String>,
    /// Avoid deriving Copy for types matching <REGEX>.
    #[arg(long, value_name = "REGEX")]
    no_copy: Vec<String>,
    /// Avoid deriving Debug for types matching <REGEX>.
    #[arg(long, value_name = "REGEX")]
    no_debug: Vec<String>,
    /// Avoid deriving/implementing Default for types matching <REGEX>.
    #[arg(long, value_name = "REGEX")]
    no_default: Vec<String>,
    /// Avoid deriving Hash for types matching <REGEX>.
    #[arg(long, value_name = "REGEX")]
    no_hash: Vec<String>,
    /// Add #[must_use] annotation to types matching <REGEX>.
    #[arg(long, value_name = "REGEX")]
    must_use_type: Vec<String>,
    /// Enables detecting unexposed attributes in functions (slow). Used to generate #[must_use] annotations.
    #[arg(long)]
    enable_function_attribute_detection: bool,
    /// Use `*const [T; size]` instead of `*const T` for C arrays
    #[arg(long)]
    use_array_pointers_in_arguments: bool,
    /// The name to be used in a #[link(wasm_import_module = ...)] statement
    #[arg(long, value_name = "NAME")]
    wasm_import_module_name: Option<String>,
    /// Use dynamic loading mode with the given library name.
    #[arg(long, value_name = "NAME")]
    dynamic_loading: Option<String>,
    /// Require successful linkage to all functions in the library.
    #[arg(long)]
    dynamic_link_require_all: bool,
    /// Makes generated bindings `pub` only for items if the items are publically accessible in C++.
    #[arg(long)]
    respect_cxx_access_specs: bool,
    /// Always translate enum integer types to native Rust integer types.
    #[arg(long)]
    translate_enum_integer_types: bool,
    /// Generate types with C style naming.
    #[arg(long)]
    c_naming: bool,
    /// Always output explicit padding fields.
    #[arg(long)]
    explicit_padding: bool,
    /// Enables generation of vtable functions.
    #[arg(long)]
    vtable_generation: bool,
    /// Enables sorting of code generation in a predefined manner.
    #[arg(long)]
    sort_semantically: bool,
    /// Deduplicates extern blocks.
    #[arg(long)]
    merge_extern_blocks: bool,
    /// Overrides the ABI of functions matching <regex>. The <OVERRIDE> value must be of the shape <REGEX>=<ABI> where <ABI> can be one of C, stdcall, fastcall, thiscall, aapcs, win64 or C-unwind.
    #[arg(long, value_name = "OVERRIDE")]
    override_abi: Vec<String>,
    /// Wrap unsafe operations in unsafe blocks.
    #[arg(long)]
    wrap_unsafe_ops: bool,
    /// Derive custom traits on any kind of type. The <CUSTOM> value must be of the shape <REGEX>=<DERIVE> where <DERIVE> is a coma-separated list of derive macros.
    #[arg(long, value_name = "CUSTOM")]
    with_derive_custom: Vec<String>,
    /// Derive custom traits on a `struct`. The <CUSTOM> value must be of the shape <REGEX>=<DERIVE> where <DERIVE> is a coma-separated list of derive macros.
    #[arg(long, value_name = "CUSTOM")]
    with_derive_custom_struct: Vec<String>,
    /// Derive custom traits on an `enum. The <CUSTOM> value must be of the shape <REGEX>=<DERIVE> where <DERIVE> is a coma-separated list of derive macros.
    #[arg(long, value_name = "CUSTOM")]
    with_derive_custom_enum: Vec<String>,
    /// Derive custom traits on a `union`. The <CUSTOM> value must be of the shape <REGEX>=<DERIVE> where <DERIVE> is a coma-separated list of derive macros.
    #[arg(long, value_name = "CUSTOM")]
    with_derive_custom_union: Vec<String>,
    /// Generate wrappers for `static` and `static inline` functions.
    #[arg(long, requires = "experimental")]
    wrap_static_fns: bool,
    /// Sets the path for the source file that must be created due to the presence of `static` and
    /// `static inline` functions.
    #[arg(long, requires = "experimental", value_name = "PATH")]
    wrap_static_fns_path: Option<PathBuf>,
    /// Sets the suffix added to the extern wrapper functions generated for `static` and `static
    /// inline` functions.
    #[arg(long, requires = "experimental", value_name = "SUFFIX")]
    wrap_static_fns_suffix: Option<String>,
    /// Enables experimental features.
    #[arg(long)]
    experimental: bool,
    /// Prints the version, and exits
    #[arg(short = 'V', long)]
    version: bool,
    /// Arguments to be passed straight through to clang.
    clang_args: Vec<String>,
}

/// Construct a new [`Builder`](./struct.Builder.html) from command line flags.
pub fn builder_from_flags<I>(
    args: I,
) -> Result<(Builder, Box<dyn io::Write>, bool), io::Error>
where
    I: Iterator<Item = String>,
{
    let command = BindgenCommand::parse_from(args);

    let BindgenCommand {
        header,
        depfile,
        default_enum_style,
        bitfield_enum,
        newtype_enum,
        newtype_global_enum,
        rustified_enum,
        constified_enum,
        constified_enum_module,
        default_macro_constant_type,
        default_alias_style,
        normal_alias,
        new_type_alias,
        new_type_alias_deref,
        default_non_copy_union_style,
        bindgen_wrapper_union,
        manually_drop_union,
        blocklist_type,
        blocklist_function,
        blocklist_item,
        blocklist_file,
        no_layout_tests,
        no_derive_copy,
        no_derive_debug,
        no_derive_default,
        impl_debug,
        impl_partialeq,
        with_derive_default,
        with_derive_hash,
        with_derive_partialeq,
        with_derive_partialord,
        with_derive_eq,
        with_derive_ord,
        no_doc_comments,
        no_recursive_allowlist,
        objc_extern_crate,
        generate_block,
        block_extern_crate,
        distrust_clang_mangling,
        builtins,
        ctypes_prefix,
        anon_fields_prefix,
        time_phases,
        emit_clang_ast,
        emit_ir,
        emit_ir_graphviz,
        enable_cxx_namespaces,
        disable_name_namespacing,
        disable_nested_struct_naming,
        disable_untagged_union,
        disable_header_comment,
        ignore_functions,
        generate,
        ignore_methods,
        no_convert_floats,
        no_prepend_enum_name,
        no_include_path_detection,
        fit_macro_constant_types,
        opaque_type,
        output,
        raw_line,
        module_raw_line,
        rust_target,
        use_core,
        conservative_inline_namespaces,
        use_msvc_mangling: _,
        allowlist_function,
        generate_inline_functions,
        allowlist_type,
        allowlist_var,
        allowlist_file,
        verbose,
        dump_preprocessed_input,
        no_record_matches,
        size_t_is_usize: _,
        no_size_t_is_usize,
        no_rustfmt_bindings,
        rustfmt_bindings: _,
        rustfmt_configuration_file,
        no_partialeq,
        no_copy,
        no_debug,
        no_default,
        no_hash,
        must_use_type,
        enable_function_attribute_detection,
        use_array_pointers_in_arguments,
        wasm_import_module_name,
        dynamic_loading,
        dynamic_link_require_all,
        respect_cxx_access_specs,
        translate_enum_integer_types,
        c_naming,
        explicit_padding,
        vtable_generation,
        sort_semantically,
        merge_extern_blocks,
        override_abi,
        wrap_unsafe_ops,
        with_derive_custom,
        with_derive_custom_struct,
        with_derive_custom_enum,
        with_derive_custom_union,
        wrap_static_fns,
        wrap_static_fns_path,
        wrap_static_fns_suffix,
        experimental: _,
        version,
        clang_args,
    } = command;

    if version {
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

    if let Some(header) = header {
        builder = builder.header(header);
    } else {
        return Err(Error::new(ErrorKind::Other, "Header not found"));
    }

    if let Some(rust_target) = rust_target {
        builder = builder.rust_target(rust_target);
    }

    if let Some(variant) = default_enum_style {
        builder = builder.default_enum_style(variant);
    }

    for regex in bitfield_enum {
        builder = builder.bitfield_enum(regex);
    }

    for regex in newtype_enum {
        builder = builder.newtype_enum(regex);
    }

    for regex in newtype_global_enum {
        builder = builder.newtype_global_enum(regex);
    }

    for regex in rustified_enum {
        builder = builder.rustified_enum(regex);
    }

    for regex in constified_enum {
        builder = builder.constified_enum(regex);
    }

    for regex in constified_enum_module {
        builder = builder.constified_enum_module(regex);
    }

    if let Some(default_macro_constant_type) = default_macro_constant_type {
        builder =
            builder.default_macro_constant_type(default_macro_constant_type)
    }

    if let Some(variant) = default_alias_style {
        builder = builder.default_alias_style(variant);
    }

    for regex in normal_alias {
        builder = builder.type_alias(regex);
    }

    for regex in new_type_alias {
        builder = builder.new_type_alias(regex);
    }

    for regex in new_type_alias_deref {
        builder = builder.new_type_alias_deref(regex);
    }

    if let Some(variant) = default_non_copy_union_style {
        builder = builder.default_non_copy_union_style(variant);
    }

    for regex in bindgen_wrapper_union {
        builder = builder.bindgen_wrapper_union(regex);
    }

    for regex in manually_drop_union {
        builder = builder.manually_drop_union(regex);
    }

    for ty in blocklist_type {
        builder = builder.blocklist_type(ty);
    }

    for fun in blocklist_function {
        builder = builder.blocklist_function(fun);
    }

    for id in blocklist_item {
        builder = builder.blocklist_item(id);
    }

    for file in blocklist_file {
        builder = builder.blocklist_file(file);
    }

    if builtins {
        builder = builder.emit_builtins();
    }

    if no_layout_tests {
        builder = builder.layout_tests(false);
    }

    if no_derive_copy {
        builder = builder.derive_copy(false);
    }

    if no_derive_debug {
        builder = builder.derive_debug(false);
    }

    if impl_debug {
        builder = builder.impl_debug(true);
    }

    if impl_partialeq {
        builder = builder.impl_partialeq(true);
    }

    if with_derive_default {
        builder = builder.derive_default(true);
    }

    if with_derive_hash {
        builder = builder.derive_hash(true);
    }

    if with_derive_partialeq {
        builder = builder.derive_partialeq(true);
    }

    if with_derive_partialord {
        builder = builder.derive_partialord(true);
    }

    if with_derive_eq {
        builder = builder.derive_eq(true);
    }

    if with_derive_ord {
        builder = builder.derive_ord(true);
    }

    if no_derive_default {
        builder = builder.derive_default(false);
    }

    if no_prepend_enum_name {
        builder = builder.prepend_enum_name(false);
    }

    if no_include_path_detection {
        builder = builder.detect_include_paths(false);
    }

    if fit_macro_constant_types {
        builder = builder.fit_macro_constants(true);
    }

    if time_phases {
        builder = builder.time_phases(true);
    }

    if use_array_pointers_in_arguments {
        builder = builder.array_pointers_in_arguments(true);
    }

    if let Some(wasm_import_name) = wasm_import_module_name {
        builder = builder.wasm_import_module_name(wasm_import_name);
    }

    if let Some(prefix) = ctypes_prefix {
        builder = builder.ctypes_prefix(prefix);
    }

    builder = builder.anon_fields_prefix(anon_fields_prefix);

    if let Some(config) = generate {
        builder = builder.with_codegen_config(config);
    }

    if emit_clang_ast {
        builder = builder.emit_clang_ast();
    }

    if emit_ir {
        builder = builder.emit_ir();
    }

    if let Some(path) = emit_ir_graphviz {
        builder = builder.emit_ir_graphviz(path);
    }

    if enable_cxx_namespaces {
        builder = builder.enable_cxx_namespaces();
    }

    if enable_function_attribute_detection {
        builder = builder.enable_function_attribute_detection();
    }

    if disable_name_namespacing {
        builder = builder.disable_name_namespacing();
    }

    if disable_nested_struct_naming {
        builder = builder.disable_nested_struct_naming();
    }

    if disable_untagged_union {
        builder = builder.disable_untagged_union();
    }

    if disable_header_comment {
        builder = builder.disable_header_comment();
    }

    if ignore_functions {
        builder = builder.ignore_functions();
    }

    if ignore_methods {
        builder = builder.ignore_methods();
    }

    if no_convert_floats {
        builder = builder.no_convert_floats();
    }

    if no_doc_comments {
        builder = builder.generate_comments(false);
    }

    if no_recursive_allowlist {
        builder = builder.allowlist_recursively(false);
    }

    if objc_extern_crate {
        builder = builder.objc_extern_crate(true);
    }

    if generate_block {
        builder = builder.generate_block(true);
    }

    if block_extern_crate {
        builder = builder.block_extern_crate(true);
    }

    for ty in opaque_type {
        builder = builder.opaque_type(ty);
    }

    for line in raw_line {
        builder = builder.raw_line(line);
    }

    let mut values = module_raw_line.into_iter();
    while let Some(module) = values.next() {
        let line = values.next().unwrap();
        builder = builder.module_raw_line(module, line);
    }

    if use_core {
        builder = builder.use_core();
    }

    if distrust_clang_mangling {
        builder = builder.trust_clang_mangling(false);
    }

    if conservative_inline_namespaces {
        builder = builder.conservative_inline_namespaces();
    }

    if generate_inline_functions {
        builder = builder.generate_inline_functions(true);
    }

    for regex in allowlist_function {
        builder = builder.allowlist_function(regex);
    }

    for regex in allowlist_type {
        builder = builder.allowlist_type(regex);
    }

    for regex in allowlist_var {
        builder = builder.allowlist_var(regex);
    }

    for file in allowlist_file {
        builder = builder.allowlist_file(file);
    }

    for arg in clang_args {
        builder = builder.clang_arg(arg);
    }

    let output = if let Some(path) = &output {
        let file = File::create(path)?;
        if let Some(depfile) = depfile {
            builder = builder.depfile(path, depfile);
        }
        Box::new(io::BufWriter::new(file)) as Box<dyn io::Write>
    } else {
        if let Some(depfile) = depfile {
            builder = builder.depfile("-", depfile);
        }
        Box::new(io::BufWriter::new(io::stdout())) as Box<dyn io::Write>
    };

    if dump_preprocessed_input {
        builder.dump_preprocessed_input()?;
    }

    if no_record_matches {
        builder = builder.record_matches(false);
    }

    if no_size_t_is_usize {
        builder = builder.size_t_is_usize(false);
    }

    if no_rustfmt_bindings {
        builder = builder.rustfmt_bindings(false);
    }

    if let Some(path_str) = rustfmt_configuration_file {
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

    for regex in no_partialeq {
        builder = builder.no_partialeq(regex);
    }

    for regex in no_copy {
        builder = builder.no_copy(regex);
    }

    for regex in no_debug {
        builder = builder.no_debug(regex);
    }

    for regex in no_default {
        builder = builder.no_default(regex);
    }

    for regex in no_hash {
        builder = builder.no_hash(regex);
    }

    for regex in must_use_type {
        builder = builder.must_use_type(regex);
    }

    if let Some(dynamic_library_name) = dynamic_loading {
        builder = builder.dynamic_library_name(dynamic_library_name);
    }

    if dynamic_link_require_all {
        builder = builder.dynamic_link_require_all(true);
    }

    if respect_cxx_access_specs {
        builder = builder.respect_cxx_access_specs(true);
    }

    if translate_enum_integer_types {
        builder = builder.translate_enum_integer_types(true);
    }

    if c_naming {
        builder = builder.c_naming(true);
    }

    if explicit_padding {
        builder = builder.explicit_padding(true);
    }

    if vtable_generation {
        builder = builder.vtable_generation(true);
    }

    if sort_semantically {
        builder = builder.sort_semantically(true);
    }

    if merge_extern_blocks {
        builder = builder.merge_extern_blocks(true);
    }

    for abi_override in override_abi {
        let (regex, abi_str) = abi_override
            .rsplit_once('=')
            .expect("Invalid ABI override: Missing `=`");
        let abi = abi_str
            .parse()
            .unwrap_or_else(|err| panic!("Invalid ABI override: {}", err));
        builder = builder.override_abi(abi, regex);
    }

    if wrap_unsafe_ops {
        builder = builder.wrap_unsafe_ops(true);
    }

    #[derive(Debug)]
    struct CustomDeriveCallback {
        derives: Vec<String>,
        kind: Option<TypeKind>,
        regex_set: bindgen::RegexSet,
    }

    impl bindgen::callbacks::ParseCallbacks for CustomDeriveCallback {
        fn cli_args(&self) -> Vec<String> {
            let mut args = vec![];

            let flag = match &self.kind {
                None => "--with-derive-custom",
                Some(TypeKind::Struct) => "--with-derive-custom-struct",
                Some(TypeKind::Enum) => "--with-derive-custom-enum",
                Some(TypeKind::Union) => "--with-derive-custom-union",
            };

            let derives = self.derives.join(",");

            for item in self.regex_set.get_items() {
                args.extend_from_slice(&[
                    flag.to_owned(),
                    format!("{}={}", item, derives),
                ]);
            }

            args
        }

        fn add_derives(
            &self,
            info: &bindgen::callbacks::DeriveInfo<'_>,
        ) -> Vec<String> {
            if self.kind.map(|kind| kind == info.kind).unwrap_or(true) &&
                self.regex_set.matches(info.name)
            {
                return self.derives.clone();
            }
            vec![]
        }
    }

    for (custom_derives, kind) in [
        (with_derive_custom, None),
        (with_derive_custom_struct, Some(TypeKind::Struct)),
        (with_derive_custom_enum, Some(TypeKind::Enum)),
        (with_derive_custom_union, Some(TypeKind::Union)),
    ] {
        for custom_derive in custom_derives {
            let (regex, derives) = custom_derive
                .rsplit_once('=')
                .expect("Invalid custom derive argument: Missing `=`");
            let derives = derives.split(',').map(|s| s.to_owned()).collect();

            let mut regex_set = RegexSet::new();
            regex_set.insert(regex);
            regex_set.build(false);

            builder = builder.parse_callbacks(Box::new(CustomDeriveCallback {
                derives,
                kind,
                regex_set,
            }));
        }
    }

    if wrap_static_fns {
        builder = builder.wrap_static_fns(true);
    }

    if let Some(path) = wrap_static_fns_path {
        builder = builder.wrap_static_fns_path(path);
    }

    if let Some(suffix) = wrap_static_fns_suffix {
        builder = builder.wrap_static_fns_suffix(suffix);
    }

    Ok((builder, output, verbose))
}
