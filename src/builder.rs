use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

use crate::{
    args_are_cpp,
    callbacks::ParseCallbacks,
    clang,
    codegen::{AliasVariation, EnumVariation, MacroTypeVariation},
    deps::DepfileSpec,
    features::{RustFeatures, RustTarget, LATEST_STABLE_RUST},
    file_is_cpp, get_extra_clang_args,
    regex_set::{RegexItems, RegexSet},
    BindgenError, BindgenState, Bindings, CodegenConfig,
    DEFAULT_ANON_FIELDS_PREFIX,
};

/// Configure and generate Rust bindings for a C/C++ header.
///
/// This is the main entry point to the library.
///
/// ```ignore
/// use bindgen::builder;
///
/// // Configure and generate bindings.
/// let bindings = builder().header("path/to/input/header")
///     .allowlist_type("SomeCoolClass")
///     .allowlist_function("do_some_cool_thing")
///     .generate()?;
///
/// // Write the generated bindings to an output file.
/// bindings.write_to_file("path/to/output.rs")?;
/// ```
///
/// # Enums
///
/// Bindgen can map C/C++ enums into Rust in different ways. The way bindgen maps enums depends on
/// the pattern passed to several methods:
///
/// 1. [`constified_enum_module()`](#method.constified_enum_module)
/// 2. [`bitfield_enum()`](#method.bitfield_enum)
/// 3. [`newtype_enum()`](#method.newtype_enum)
/// 4. [`rustified_enum()`](#method.rustified_enum)
///
/// For each C enum, bindgen tries to match the pattern in the following order:
///
/// 1. Constified enum module
/// 2. Bitfield enum
/// 3. Newtype enum
/// 4. Rustified enum
///
/// If none of the above patterns match, then bindgen will generate a set of Rust constants.
///
/// # Clang arguments
///
/// Extra arguments can be passed to with clang:
/// 1. [`clang_arg()`](#method.clang_arg): takes a single argument
/// 2. [`clang_args()`](#method.clang_args): takes an iterator of arguments
/// 3. `BINDGEN_EXTRA_CLANG_ARGS` environment variable: whitespace separate
///    environment variable of arguments
///
/// Clang arguments specific to your crate should be added via the
/// `clang_arg()`/`clang_args()` methods.
///
/// End-users of the crate may need to set the `BINDGEN_EXTRA_CLANG_ARGS` environment variable to
/// add additional arguments. For example, to build against a different sysroot a user could set
/// `BINDGEN_EXTRA_CLANG_ARGS` to `--sysroot=/path/to/sysroot`.
#[derive(Debug, Default)]
pub struct Builder {
    parse_callbacks: Option<Box<dyn ParseCallbacks>>,
    inputs: BindgenInputs,
}

impl Builder {
    /// Generates the command line flags use for creating `Builder`.
    pub fn command_line_flags(&self) -> Vec<String> {
        let mut output_vector: Vec<String> = Vec::new();

        if let Some(header) = self.inputs.input_headers.last().cloned() {
            // Positional argument 'header'
            output_vector.push(header);
        }

        output_vector.push("--rust-target".into());
        output_vector.push(self.inputs.rust_target.into());

        // FIXME(emilio): This is a bit hacky, maybe we should stop re-using the
        // RustFeatures to store the "disable_untagged_union" call, and make it
        // a different flag that we check elsewhere / in generate().
        if !self.inputs.rust_features.untagged_union &&
            RustFeatures::from(self.inputs.rust_target).untagged_union
        {
            output_vector.push("--disable-untagged-union".into());
        }

        if self.inputs.default_enum_style != Default::default() {
            output_vector.push("--default-enum-style".into());
            output_vector.push(
                match self.inputs.default_enum_style {
                    EnumVariation::Rust {
                        non_exhaustive: false,
                    } => "rust",
                    EnumVariation::Rust {
                        non_exhaustive: true,
                    } => "rust_non_exhaustive",
                    EnumVariation::NewType {
                        is_bitfield: true, ..
                    } => "bitfield",
                    EnumVariation::NewType {
                        is_bitfield: false,
                        is_global,
                    } => {
                        if is_global {
                            "newtype_global"
                        } else {
                            "newtype"
                        }
                    }
                    EnumVariation::Consts => "consts",
                    EnumVariation::ModuleConsts => "moduleconsts",
                }
                .into(),
            )
        }

        if self.inputs.default_macro_constant_type != Default::default() {
            output_vector.push("--default-macro-constant-type".into());
            output_vector
                .push(self.inputs.default_macro_constant_type.as_str().into());
        }

        if self.inputs.default_alias_style != Default::default() {
            output_vector.push("--default-alias-style".into());
            output_vector.push(self.inputs.default_alias_style.as_str().into());
        }

        if self.inputs.default_non_copy_union_style != Default::default() {
            output_vector.push("--default-non-copy-union-style".into());
            output_vector
                .push(self.inputs.default_non_copy_union_style.as_str().into());
        }

        let regex_items = &[
            (&self.inputs.bitfield_enums, "--bitfield-enum"),
            (&self.inputs.newtype_enums, "--newtype-enum"),
            (&self.inputs.newtype_global_enums, "--newtype-global-enum"),
            (&self.inputs.rustified_enums, "--rustified-enum"),
            (
                &self.inputs.rustified_non_exhaustive_enums,
                "--rustified-enum-non-exhaustive",
            ),
            (
                &self.inputs.constified_enum_modules,
                "--constified-enum-module",
            ),
            (&self.inputs.constified_enums, "--constified-enum"),
            (&self.inputs.type_alias, "--type-alias"),
            (&self.inputs.new_type_alias, "--new-type-alias"),
            (&self.inputs.new_type_alias_deref, "--new-type-alias-deref"),
            (
                &self.inputs.bindgen_wrapper_union,
                "--bindgen-wrapper-union",
            ),
            (&self.inputs.manually_drop_union, "--manually-drop-union"),
            (&self.inputs.blocklisted_types, "--blocklist-type"),
            (&self.inputs.blocklisted_functions, "--blocklist-function"),
            (&self.inputs.blocklisted_items, "--blocklist-item"),
            (&self.inputs.blocklisted_files, "--blocklist-file"),
            (&self.inputs.opaque_types, "--opaque-type"),
            (&self.inputs.allowlisted_functions, "--allowlist-function"),
            (&self.inputs.allowlisted_types, "--allowlist-type"),
            (&self.inputs.allowlisted_vars, "--allowlist-var"),
            (&self.inputs.allowlisted_files, "--allowlist-file"),
            (&self.inputs.no_partialeq_types, "--no-partialeq"),
            (&self.inputs.no_copy_types, "--no-copy"),
            (&self.inputs.no_debug_types, "--no-debug"),
            (&self.inputs.no_default_types, "--no-default"),
            (&self.inputs.no_hash_types, "--no-hash"),
            (&self.inputs.must_use_types, "--must-use-type"),
        ];

        for (items, flag) in regex_items {
            for item in items.get_items() {
                output_vector.push((*flag).to_owned());
                output_vector.push(item.to_owned());
            }
        }

        if !self.inputs.layout_tests {
            output_vector.push("--no-layout-tests".into());
        }

        if self.inputs.impl_debug {
            output_vector.push("--impl-debug".into());
        }

        if self.inputs.impl_partialeq {
            output_vector.push("--impl-partialeq".into());
        }

        if !self.inputs.derive_copy {
            output_vector.push("--no-derive-copy".into());
        }

        if !self.inputs.derive_debug {
            output_vector.push("--no-derive-debug".into());
        }

        if self.inputs.derive_default {
            output_vector.push("--with-derive-default".into());
        } else {
            output_vector.push("--no-derive-default".into());
        }

        if self.inputs.derive_hash {
            output_vector.push("--with-derive-hash".into());
        }

        if self.inputs.derive_partialord {
            output_vector.push("--with-derive-partialord".into());
        }

        if self.inputs.derive_ord {
            output_vector.push("--with-derive-ord".into());
        }

        if self.inputs.derive_partialeq {
            output_vector.push("--with-derive-partialeq".into());
        }

        if self.inputs.derive_eq {
            output_vector.push("--with-derive-eq".into());
        }

        if self.inputs.time_phases {
            output_vector.push("--time-phases".into());
        }

        if !self.inputs.generate_comments {
            output_vector.push("--no-doc-comments".into());
        }

        if !self.inputs.allowlist_recursively {
            output_vector.push("--no-recursive-allowlist".into());
        }

        if self.inputs.objc_extern_crate {
            output_vector.push("--objc-extern-crate".into());
        }

        if self.inputs.generate_block {
            output_vector.push("--generate-block".into());
        }

        if self.inputs.block_extern_crate {
            output_vector.push("--block-extern-crate".into());
        }

        if self.inputs.builtins {
            output_vector.push("--builtins".into());
        }

        if let Some(ref prefix) = self.inputs.ctypes_prefix {
            output_vector.push("--ctypes-prefix".into());
            output_vector.push(prefix.clone());
        }

        if self.inputs.anon_fields_prefix != DEFAULT_ANON_FIELDS_PREFIX {
            output_vector.push("--anon-fields-prefix".into());
            output_vector.push(self.inputs.anon_fields_prefix.clone());
        }

        if self.inputs.emit_ast {
            output_vector.push("--emit-clang-ast".into());
        }

        if self.inputs.emit_ir {
            output_vector.push("--emit-ir".into());
        }
        if let Some(ref graph) = self.inputs.emit_ir_graphviz {
            output_vector.push("--emit-ir-graphviz".into());
            output_vector.push(graph.clone())
        }
        if self.inputs.enable_cxx_namespaces {
            output_vector.push("--enable-cxx-namespaces".into());
        }
        if self.inputs.enable_function_attribute_detection {
            output_vector.push("--enable-function-attribute-detection".into());
        }
        if !self.inputs.enable_name_namespacing {
            output_vector.push("--disable-name-namespacing".into());
        }
        if !self.inputs.enable_nested_struct_naming {
            output_vector.push("--disable-nested-struct-naming".into());
        }

        if !self.inputs.enable_header_comment {
            output_vector.push("--disable-header-comment".into());
        }

        if !self.inputs.codegen_config.functions() {
            output_vector.push("--ignore-functions".into());
        }

        output_vector.push("--generate".into());

        //Temporary placeholder for below 4 options
        let mut options: Vec<String> = Vec::new();
        if self.inputs.codegen_config.functions() {
            options.push("functions".into());
        }
        if self.inputs.codegen_config.types() {
            options.push("types".into());
        }
        if self.inputs.codegen_config.vars() {
            options.push("vars".into());
        }
        if self.inputs.codegen_config.methods() {
            options.push("methods".into());
        }
        if self.inputs.codegen_config.constructors() {
            options.push("constructors".into());
        }
        if self.inputs.codegen_config.destructors() {
            options.push("destructors".into());
        }

        output_vector.push(options.join(","));

        if !self.inputs.codegen_config.methods() {
            output_vector.push("--ignore-methods".into());
        }

        if !self.inputs.convert_floats {
            output_vector.push("--no-convert-floats".into());
        }

        if !self.inputs.prepend_enum_name {
            output_vector.push("--no-prepend-enum-name".into());
        }

        if self.inputs.fit_macro_constants {
            output_vector.push("--fit-macro-constant-types".into());
        }

        if self.inputs.array_pointers_in_arguments {
            output_vector.push("--use-array-pointers-in-arguments".into());
        }

        if let Some(ref wasm_import_module_name) =
            self.inputs.wasm_import_module_name
        {
            output_vector.push("--wasm-import-module-name".into());
            output_vector.push(wasm_import_module_name.clone());
        }

        for line in &self.inputs.raw_lines {
            output_vector.push("--raw-line".into());
            output_vector.push(line.clone());
        }

        for (module, lines) in &self.inputs.module_lines {
            for line in lines.iter() {
                output_vector.push("--module-raw-line".into());
                output_vector.push(module.clone());
                output_vector.push(line.clone());
            }
        }

        if self.inputs.use_core {
            output_vector.push("--use-core".into());
        }

        if self.inputs.conservative_inline_namespaces {
            output_vector.push("--conservative-inline-namespaces".into());
        }

        if self.inputs.generate_inline_functions {
            output_vector.push("--generate-inline-functions".into());
        }

        if !self.inputs.record_matches {
            output_vector.push("--no-record-matches".into());
        }

        if self.inputs.size_t_is_usize {
            output_vector.push("--size_t-is-usize".into());
        }

        if !self.inputs.rustfmt_bindings {
            output_vector.push("--no-rustfmt-bindings".into());
        }

        if let Some(path) = self
            .inputs
            .rustfmt_configuration_file
            .as_ref()
            .and_then(|f| f.to_str())
        {
            output_vector.push("--rustfmt-configuration-file".into());
            output_vector.push(path.into());
        }

        if let Some(ref name) = self.inputs.dynamic_library_name {
            output_vector.push("--dynamic-loading".into());
            output_vector.push(name.clone());
        }

        if self.inputs.dynamic_link_require_all {
            output_vector.push("--dynamic-link-require-all".into());
        }

        if self.inputs.respect_cxx_access_specs {
            output_vector.push("--respect-cxx-access-specs".into());
        }

        if self.inputs.translate_enum_integer_types {
            output_vector.push("--translate-enum-integer-types".into());
        }

        if self.inputs.c_naming {
            output_vector.push("--c-naming".into());
        }

        if self.inputs.force_explicit_padding {
            output_vector.push("--explicit-padding".into());
        }

        if self.inputs.vtable_generation {
            output_vector.push("--vtable-generation".into());
        }

        if self.inputs.sort_semantically {
            output_vector.push("--sort-semantically".into());
        }

        if self.inputs.merge_extern_blocks {
            output_vector.push("--merge-extern-blocks".into());
        }

        // Add clang arguments

        output_vector.push("--".into());

        if !self.inputs.clang_args.is_empty() {
            output_vector.extend(self.inputs.clang_args.iter().cloned());
        }

        if self.inputs.input_headers.len() > 1 {
            // To pass more than one header, we need to pass all but the last
            // header via the `-include` clang arg
            for header in &self.inputs.input_headers
                [..self.inputs.input_headers.len() - 1]
            {
                output_vector.push("-include".to_string());
                output_vector.push(header.clone());
            }
        }

        output_vector
    }

    /// Add an input C/C++ header to generate bindings for.
    ///
    /// This can be used to generate bindings to a single header:
    ///
    /// ```ignore
    /// let bindings = bindgen::Builder::default()
    ///     .header("input.h")
    ///     .generate()
    ///     .unwrap();
    /// ```
    ///
    /// Or you can invoke it multiple times to generate bindings to multiple
    /// headers:
    ///
    /// ```ignore
    /// let bindings = bindgen::Builder::default()
    ///     .header("first.h")
    ///     .header("second.h")
    ///     .header("third.h")
    ///     .generate()
    ///     .unwrap();
    /// ```
    pub fn header<T: Into<String>>(mut self, header: T) -> Builder {
        self.inputs.input_headers.push(header.into());
        self
    }

    /// Add a depfile output which will be written alongside the generated bindings.
    pub fn depfile<H: Into<String>, D: Into<PathBuf>>(
        mut self,
        output_module: H,
        depfile: D,
    ) -> Builder {
        self.inputs.depfile = Some(DepfileSpec {
            output_module: output_module.into(),
            depfile_path: depfile.into(),
        });
        self
    }

    /// Add `contents` as an input C/C++ header named `name`.
    ///
    /// The file `name` will be added to the clang arguments.
    pub fn header_contents(mut self, name: &str, contents: &str) -> Builder {
        // Apparently clang relies on having virtual FS correspondent to
        // the real one, so we need absolute paths here
        let absolute_path = env::current_dir()
            .expect("Cannot retrieve current directory")
            .join(name)
            .to_str()
            .expect("Cannot convert current directory name to string")
            .to_owned();
        self.inputs
            .input_header_contents
            .push((absolute_path, contents.into()));
        self
    }

    /// Specify the rust target
    ///
    /// The default is the latest stable Rust version
    pub fn rust_target(mut self, rust_target: RustTarget) -> Self {
        self.inputs.set_rust_target(rust_target);
        self
    }

    /// Disable support for native Rust unions, if supported.
    pub fn disable_untagged_union(mut self) -> Self {
        self.inputs.rust_features.untagged_union = false;
        self
    }

    /// Disable insertion of bindgen's version identifier into generated
    /// bindings.
    pub fn disable_header_comment(mut self) -> Self {
        self.inputs.enable_header_comment = false;
        self
    }

    /// Set the output graphviz file.
    pub fn emit_ir_graphviz<T: Into<String>>(mut self, path: T) -> Builder {
        let path = path.into();
        self.inputs.emit_ir_graphviz = Some(path);
        self
    }

    /// Whether the generated bindings should contain documentation comments
    /// (docstrings) or not. This is set to true by default.
    ///
    /// Note that clang by default excludes comments from system headers, pass
    /// `-fretain-comments-from-system-headers` as
    /// [`clang_arg`][Builder::clang_arg] to include them. It can also be told
    /// to process all comments (not just documentation ones) using the
    /// `-fparse-all-comments` flag. See [slides on clang comment parsing](
    /// https://llvm.org/devmtg/2012-11/Gribenko_CommentParsing.pdf) for
    /// background and examples.
    pub fn generate_comments(mut self, doit: bool) -> Self {
        self.inputs.generate_comments = doit;
        self
    }

    /// Whether to allowlist recursively or not. Defaults to true.
    ///
    /// Given that we have explicitly allowlisted the "initiate_dance_party"
    /// function in this C header:
    ///
    /// ```c
    /// typedef struct MoonBoots {
    ///     int bouncy_level;
    /// } MoonBoots;
    ///
    /// void initiate_dance_party(MoonBoots* boots);
    /// ```
    ///
    /// We would normally generate bindings to both the `initiate_dance_party`
    /// function and the `MoonBoots` struct that it transitively references. By
    /// configuring with `allowlist_recursively(false)`, `bindgen` will not emit
    /// bindings for anything except the explicitly allowlisted items, and there
    /// would be no emitted struct definition for `MoonBoots`. However, the
    /// `initiate_dance_party` function would still reference `MoonBoots`!
    ///
    /// **Disabling this feature will almost certainly cause `bindgen` to emit
    /// bindings that will not compile!** If you disable this feature, then it
    /// is *your* responsibility to provide definitions for every type that is
    /// referenced from an explicitly allowlisted item. One way to provide the
    /// definitions is by using the [`Builder::raw_line`](#method.raw_line)
    /// method, another would be to define them in Rust and then `include!(...)`
    /// the bindings immediately afterwards.
    pub fn allowlist_recursively(mut self, doit: bool) -> Self {
        self.inputs.allowlist_recursively = doit;
        self
    }

    /// Deprecated alias for allowlist_recursively.
    #[deprecated(note = "Use allowlist_recursively instead")]
    pub fn whitelist_recursively(self, doit: bool) -> Self {
        self.allowlist_recursively(doit)
    }

    /// Generate `#[macro_use] extern crate objc;` instead of `use objc;`
    /// in the prologue of the files generated from objective-c files
    pub fn objc_extern_crate(mut self, doit: bool) -> Self {
        self.inputs.objc_extern_crate = doit;
        self
    }

    /// Generate proper block signatures instead of void pointers.
    pub fn generate_block(mut self, doit: bool) -> Self {
        self.inputs.generate_block = doit;
        self
    }

    /// Generate `#[macro_use] extern crate block;` instead of `use block;`
    /// in the prologue of the files generated from apple block files
    pub fn block_extern_crate(mut self, doit: bool) -> Self {
        self.inputs.block_extern_crate = doit;
        self
    }

    /// Whether to use the clang-provided name mangling. This is true by default
    /// and probably needed for C++ features.
    ///
    /// However, some old libclang versions seem to return incorrect results in
    /// some cases for non-mangled functions, see [1], so we allow disabling it.
    ///
    /// [1]: https://github.com/rust-lang/rust-bindgen/issues/528
    pub fn trust_clang_mangling(mut self, doit: bool) -> Self {
        self.inputs.enable_mangling = doit;
        self
    }

    /// Hide the given type from the generated bindings. Regular expressions are
    /// supported.
    #[deprecated(note = "Use blocklist_type instead")]
    pub fn hide_type<T: AsRef<str>>(self, arg: T) -> Builder {
        self.blocklist_type(arg)
    }

    /// Hide the given type from the generated bindings. Regular expressions are
    /// supported.
    #[deprecated(note = "Use blocklist_type instead")]
    pub fn blacklist_type<T: AsRef<str>>(self, arg: T) -> Builder {
        self.blocklist_type(arg)
    }

    /// Hide the given type from the generated bindings. Regular expressions are
    /// supported.
    ///
    /// To blocklist types prefixed with "mylib" use `"mylib_.*"`.
    /// For more complicated expressions check
    /// [regex](https://docs.rs/regex/*/regex/) docs
    pub fn blocklist_type<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.inputs.blocklisted_types.insert(arg);
        self
    }

    /// Hide the given function from the generated bindings. Regular expressions
    /// are supported.
    #[deprecated(note = "Use blocklist_function instead")]
    pub fn blacklist_function<T: AsRef<str>>(self, arg: T) -> Builder {
        self.blocklist_function(arg)
    }

    /// Hide the given function from the generated bindings. Regular expressions
    /// are supported.
    ///
    /// To blocklist functions prefixed with "mylib" use `"mylib_.*"`.
    /// For more complicated expressions check
    /// [regex](https://docs.rs/regex/*/regex/) docs
    pub fn blocklist_function<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.inputs.blocklisted_functions.insert(arg);
        self
    }

    /// Hide the given item from the generated bindings, regardless of
    /// whether it's a type, function, module, etc. Regular
    /// expressions are supported.
    #[deprecated(note = "Use blocklist_item instead")]
    pub fn blacklist_item<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.inputs.blocklisted_items.insert(arg);
        self
    }

    /// Hide the given item from the generated bindings, regardless of
    /// whether it's a type, function, module, etc. Regular
    /// expressions are supported.
    ///
    /// To blocklist items prefixed with "mylib" use `"mylib_.*"`.
    /// For more complicated expressions check
    /// [regex](https://docs.rs/regex/*/regex/) docs
    pub fn blocklist_item<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.inputs.blocklisted_items.insert(arg);
        self
    }

    /// Hide any contents of the given file from the generated bindings,
    /// regardless of whether it's a type, function, module etc.
    pub fn blocklist_file<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.inputs.blocklisted_files.insert(arg);
        self
    }

    /// Treat the given type as opaque in the generated bindings. Regular
    /// expressions are supported.
    ///
    /// To change types prefixed with "mylib" into opaque, use `"mylib_.*"`.
    /// For more complicated expressions check
    /// [regex](https://docs.rs/regex/*/regex/) docs
    pub fn opaque_type<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.inputs.opaque_types.insert(arg);
        self
    }

    /// Allowlist the given type so that it (and all types that it transitively
    /// refers to) appears in the generated bindings. Regular expressions are
    /// supported.
    #[deprecated(note = "use allowlist_type instead")]
    pub fn whitelisted_type<T: AsRef<str>>(self, arg: T) -> Builder {
        self.allowlist_type(arg)
    }

    /// Allowlist the given type so that it (and all types that it transitively
    /// refers to) appears in the generated bindings. Regular expressions are
    /// supported.
    #[deprecated(note = "use allowlist_type instead")]
    pub fn whitelist_type<T: AsRef<str>>(self, arg: T) -> Builder {
        self.allowlist_type(arg)
    }

    /// Allowlist the given type so that it (and all types that it transitively
    /// refers to) appears in the generated bindings. Regular expressions are
    /// supported.
    ///
    /// To allowlist types prefixed with "mylib" use `"mylib_.*"`.
    /// For more complicated expressions check
    /// [regex](https://docs.rs/regex/*/regex/) docs
    pub fn allowlist_type<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.inputs.allowlisted_types.insert(arg);
        self
    }

    /// Allowlist the given function so that it (and all types that it
    /// transitively refers to) appears in the generated bindings. Regular
    /// expressions are supported.
    ///
    /// To allowlist functions prefixed with "mylib" use `"mylib_.*"`.
    /// For more complicated expressions check
    /// [regex](https://docs.rs/regex/*/regex/) docs
    pub fn allowlist_function<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.inputs.allowlisted_functions.insert(arg);
        self
    }

    /// Allowlist the given function.
    ///
    /// Deprecated: use allowlist_function instead.
    #[deprecated(note = "use allowlist_function instead")]
    pub fn whitelist_function<T: AsRef<str>>(self, arg: T) -> Builder {
        self.allowlist_function(arg)
    }

    /// Allowlist the given function.
    ///
    /// Deprecated: use allowlist_function instead.
    #[deprecated(note = "use allowlist_function instead")]
    pub fn whitelisted_function<T: AsRef<str>>(self, arg: T) -> Builder {
        self.allowlist_function(arg)
    }

    /// Allowlist the given variable so that it (and all types that it
    /// transitively refers to) appears in the generated bindings. Regular
    /// expressions are supported.
    ///
    /// To allowlist variables prefixed with "mylib" use `"mylib_.*"`.
    /// For more complicated expressions check
    /// [regex](https://docs.rs/regex/*/regex/) docs
    pub fn allowlist_var<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.inputs.allowlisted_vars.insert(arg);
        self
    }

    /// Allowlist the given file so that its contents appear in the generated bindings.
    pub fn allowlist_file<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.inputs.allowlisted_files.insert(arg);
        self
    }

    /// Deprecated: use allowlist_var instead.
    #[deprecated(note = "use allowlist_var instead")]
    pub fn whitelist_var<T: AsRef<str>>(self, arg: T) -> Builder {
        self.allowlist_var(arg)
    }

    /// Allowlist the given variable.
    ///
    /// Deprecated: use allowlist_var instead.
    #[deprecated(note = "use allowlist_var instead")]
    pub fn whitelisted_var<T: AsRef<str>>(self, arg: T) -> Builder {
        self.allowlist_var(arg)
    }

    /// Set the default style of code to generate for enums
    pub fn default_enum_style(mut self, arg: EnumVariation) -> Builder {
        self.inputs.default_enum_style = arg;
        self
    }

    /// Mark the given enum (or set of enums, if using a pattern) as being
    /// bitfield-like. Regular expressions are supported.
    ///
    /// This makes bindgen generate a type that isn't a rust `enum`. Regular
    /// expressions are supported.
    ///
    /// This is similar to the newtype enum style, but with the bitwise
    /// operators implemented.
    pub fn bitfield_enum<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.inputs.bitfield_enums.insert(arg);
        self
    }

    /// Mark the given enum (or set of enums, if using a pattern) as a newtype.
    /// Regular expressions are supported.
    ///
    /// This makes bindgen generate a type that isn't a Rust `enum`. Regular
    /// expressions are supported.
    pub fn newtype_enum<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.inputs.newtype_enums.insert(arg);
        self
    }

    /// Mark the given enum (or set of enums, if using a pattern) as a newtype
    /// whose variants are exposed as global constants.
    ///
    /// Regular expressions are supported.
    ///
    /// This makes bindgen generate a type that isn't a Rust `enum`. Regular
    /// expressions are supported.
    pub fn newtype_global_enum<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.inputs.newtype_global_enums.insert(arg);
        self
    }

    /// Mark the given enum (or set of enums, if using a pattern) as a Rust
    /// enum.
    ///
    /// This makes bindgen generate enums instead of constants. Regular
    /// expressions are supported.
    ///
    /// **Use this with caution**, creating this in unsafe code
    /// (including FFI) with an invalid value will invoke undefined behaviour.
    /// You may want to use the newtype enum style instead.
    pub fn rustified_enum<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.inputs.rustified_enums.insert(arg);
        self
    }

    /// Mark the given enum (or set of enums, if using a pattern) as a Rust
    /// enum with the `#[non_exhaustive]` attribute.
    ///
    /// This makes bindgen generate enums instead of constants. Regular
    /// expressions are supported.
    ///
    /// **Use this with caution**, creating this in unsafe code
    /// (including FFI) with an invalid value will invoke undefined behaviour.
    /// You may want to use the newtype enum style instead.
    pub fn rustified_non_exhaustive_enum<T: AsRef<str>>(
        mut self,
        arg: T,
    ) -> Builder {
        self.inputs.rustified_non_exhaustive_enums.insert(arg);
        self
    }

    /// Mark the given enum (or set of enums, if using a pattern) as a set of
    /// constants that are not to be put into a module.
    pub fn constified_enum<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.inputs.constified_enums.insert(arg);
        self
    }

    /// Mark the given enum (or set of enums, if using a pattern) as a set of
    /// constants that should be put into a module.
    ///
    /// This makes bindgen generate modules containing constants instead of
    /// just constants. Regular expressions are supported.
    pub fn constified_enum_module<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.inputs.constified_enum_modules.insert(arg);
        self
    }

    /// Set the default type for macro constants
    pub fn default_macro_constant_type(
        mut self,
        arg: MacroTypeVariation,
    ) -> Builder {
        self.inputs.default_macro_constant_type = arg;
        self
    }

    /// Set the default style of code to generate for typedefs
    pub fn default_alias_style(mut self, arg: AliasVariation) -> Builder {
        self.inputs.default_alias_style = arg;
        self
    }

    /// Mark the given typedef alias (or set of aliases, if using a pattern) to
    /// use regular Rust type aliasing.
    ///
    /// This is the default behavior and should be used if `default_alias_style`
    /// was set to NewType or NewTypeDeref and you want to override it for a
    /// set of typedefs.
    pub fn type_alias<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.inputs.type_alias.insert(arg);
        self
    }

    /// Mark the given typedef alias (or set of aliases, if using a pattern) to
    /// be generated as a new type by having the aliased type be wrapped in a
    /// #[repr(transparent)] struct.
    ///
    /// Used to enforce stricter type checking.
    pub fn new_type_alias<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.inputs.new_type_alias.insert(arg);
        self
    }

    /// Mark the given typedef alias (or set of aliases, if using a pattern) to
    /// be generated as a new type by having the aliased type be wrapped in a
    /// #[repr(transparent)] struct and also have an automatically generated
    /// impl's of `Deref` and `DerefMut` to their aliased type.
    pub fn new_type_alias_deref<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.inputs.new_type_alias_deref.insert(arg);
        self
    }
    /// Set the default style of code to generate for unions with a non-Copy member.
    pub fn default_non_copy_union_style(
        mut self,
        arg: crate::codegen::NonCopyUnionStyle,
    ) -> Self {
        self.inputs.default_non_copy_union_style = arg;
        self
    }

    /// Mark the given union (or set of union, if using a pattern) to use
    /// a bindgen-generated wrapper for its members if at least one is non-Copy.
    pub fn bindgen_wrapper_union<T: AsRef<str>>(mut self, arg: T) -> Self {
        self.inputs.bindgen_wrapper_union.insert(arg);
        self
    }

    /// Mark the given union (or set of union, if using a pattern) to use
    /// [`::core::mem::ManuallyDrop`] for its members if at least one is non-Copy.
    ///
    /// Note: `ManuallyDrop` was stabilized in Rust 1.20.0, do not use it if your
    /// MSRV is lower.
    pub fn manually_drop_union<T: AsRef<str>>(mut self, arg: T) -> Self {
        self.inputs.manually_drop_union.insert(arg);
        self
    }

    /// Add a string to prepend to the generated bindings. The string is passed
    /// through without any modification.
    pub fn raw_line<T: Into<String>>(mut self, arg: T) -> Self {
        self.inputs.raw_lines.push(arg.into());
        self
    }

    /// Add a given line to the beginning of module `mod`.
    pub fn module_raw_line<T, U>(mut self, mod_: T, line: U) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        self.inputs
            .module_lines
            .entry(mod_.into())
            .or_insert_with(Vec::new)
            .push(line.into());
        self
    }

    /// Add a given set of lines to the beginning of module `mod`.
    pub fn module_raw_lines<T, I>(mut self, mod_: T, lines: I) -> Self
    where
        T: Into<String>,
        I: IntoIterator,
        I::Item: Into<String>,
    {
        self.inputs
            .module_lines
            .entry(mod_.into())
            .or_insert_with(Vec::new)
            .extend(lines.into_iter().map(Into::into));
        self
    }

    /// Add an argument to be passed straight through to clang.
    pub fn clang_arg<T: Into<String>>(mut self, arg: T) -> Builder {
        self.inputs.clang_args.push(arg.into());
        self
    }

    /// Add arguments to be passed straight through to clang.
    pub fn clang_args<I>(mut self, iter: I) -> Builder
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        for arg in iter {
            self = self.clang_arg(arg.as_ref())
        }
        self
    }

    /// Emit bindings for builtin definitions (for example `__builtin_va_list`)
    /// in the generated Rust.
    pub fn emit_builtins(mut self) -> Builder {
        self.inputs.builtins = true;
        self
    }

    /// Avoid converting floats to `f32`/`f64` by default.
    pub fn no_convert_floats(mut self) -> Self {
        self.inputs.convert_floats = false;
        self
    }

    /// Set whether layout tests should be generated.
    pub fn layout_tests(mut self, doit: bool) -> Self {
        self.inputs.layout_tests = doit;
        self
    }

    /// Set whether `Debug` should be implemented, if it can not be derived automatically.
    pub fn impl_debug(mut self, doit: bool) -> Self {
        self.inputs.impl_debug = doit;
        self
    }

    /// Set whether `PartialEq` should be implemented, if it can not be derived automatically.
    pub fn impl_partialeq(mut self, doit: bool) -> Self {
        self.inputs.impl_partialeq = doit;
        self
    }

    /// Set whether `Copy` should be derived by default.
    pub fn derive_copy(mut self, doit: bool) -> Self {
        self.inputs.derive_copy = doit;
        self
    }

    /// Set whether `Debug` should be derived by default.
    pub fn derive_debug(mut self, doit: bool) -> Self {
        self.inputs.derive_debug = doit;
        self
    }

    /// Set whether `Default` should be derived by default.
    pub fn derive_default(mut self, doit: bool) -> Self {
        self.inputs.derive_default = doit;
        self
    }

    /// Set whether `Hash` should be derived by default.
    pub fn derive_hash(mut self, doit: bool) -> Self {
        self.inputs.derive_hash = doit;
        self
    }

    /// Set whether `PartialOrd` should be derived by default.
    /// If we don't compute partialord, we also cannot compute
    /// ord. Set the derive_ord to `false` when doit is `false`.
    pub fn derive_partialord(mut self, doit: bool) -> Self {
        self.inputs.derive_partialord = doit;
        if !doit {
            self.inputs.derive_ord = false;
        }
        self
    }

    /// Set whether `Ord` should be derived by default.
    /// We can't compute `Ord` without computing `PartialOrd`,
    /// so we set the same option to derive_partialord.
    pub fn derive_ord(mut self, doit: bool) -> Self {
        self.inputs.derive_ord = doit;
        self.inputs.derive_partialord = doit;
        self
    }

    /// Set whether `PartialEq` should be derived by default.
    ///
    /// If we don't derive `PartialEq`, we also cannot derive `Eq`, so deriving
    /// `Eq` is also disabled when `doit` is `false`.
    pub fn derive_partialeq(mut self, doit: bool) -> Self {
        self.inputs.derive_partialeq = doit;
        if !doit {
            self.inputs.derive_eq = false;
        }
        self
    }

    /// Set whether `Eq` should be derived by default.
    ///
    /// We can't derive `Eq` without also deriving `PartialEq`, so we also
    /// enable deriving `PartialEq` when `doit` is `true`.
    pub fn derive_eq(mut self, doit: bool) -> Self {
        self.inputs.derive_eq = doit;
        if doit {
            self.inputs.derive_partialeq = doit;
        }
        self
    }

    /// Set whether or not to time bindgen phases, and print information to
    /// stderr.
    pub fn time_phases(mut self, doit: bool) -> Self {
        self.inputs.time_phases = doit;
        self
    }

    /// Emit Clang AST.
    pub fn emit_clang_ast(mut self) -> Builder {
        self.inputs.emit_ast = true;
        self
    }

    /// Emit IR.
    pub fn emit_ir(mut self) -> Builder {
        self.inputs.emit_ir = true;
        self
    }

    /// Enable C++ namespaces.
    pub fn enable_cxx_namespaces(mut self) -> Builder {
        self.inputs.enable_cxx_namespaces = true;
        self
    }

    /// Enable detecting must_use attributes on C functions.
    ///
    /// This is quite slow in some cases (see #1465), so it's disabled by
    /// default.
    ///
    /// Note that for this to do something meaningful for now at least, the rust
    /// target version has to have support for `#[must_use]`.
    pub fn enable_function_attribute_detection(mut self) -> Self {
        self.inputs.enable_function_attribute_detection = true;
        self
    }

    /// Disable name auto-namespacing.
    ///
    /// By default, bindgen mangles names like `foo::bar::Baz` to look like
    /// `foo_bar_Baz` instead of just `Baz`.
    ///
    /// This method disables that behavior.
    ///
    /// Note that this intentionally does not change the names used for
    /// allowlisting and blocklisting, which should still be mangled with the
    /// namespaces.
    ///
    /// Note, also, that this option may cause bindgen to generate duplicate
    /// names.
    pub fn disable_name_namespacing(mut self) -> Builder {
        self.inputs.enable_name_namespacing = false;
        self
    }

    /// Disable nested struct naming.
    ///
    /// The following structs have different names for C and C++. In case of C
    /// they are visible as `foo` and `bar`. In case of C++ they are visible as
    /// `foo` and `foo::bar`.
    ///
    /// ```c
    /// struct foo {
    ///     struct bar {
    ///     } b;
    /// };
    /// ```
    ///
    /// Bindgen wants to avoid duplicate names by default so it follows C++ naming
    /// and it generates `foo`/`foo_bar` instead of just `foo`/`bar`.
    ///
    /// This method disables this behavior and it is indented to be used only
    /// for headers that were written for C.
    pub fn disable_nested_struct_naming(mut self) -> Builder {
        self.inputs.enable_nested_struct_naming = false;
        self
    }

    /// Treat inline namespaces conservatively.
    ///
    /// This is tricky, because in C++ is technically legal to override an item
    /// defined in an inline namespace:
    ///
    /// ```cpp
    /// inline namespace foo {
    ///     using Bar = int;
    /// }
    /// using Bar = long;
    /// ```
    ///
    /// Even though referencing `Bar` is a compiler error.
    ///
    /// We want to support this (arguably esoteric) use case, but we don't want
    /// to make the rest of bindgen users pay an usability penalty for that.
    ///
    /// To support this, we need to keep all the inline namespaces around, but
    /// then bindgen usage is a bit more difficult, because you cannot
    /// reference, e.g., `std::string` (you'd need to use the proper inline
    /// namespace).
    ///
    /// We could complicate a lot of the logic to detect name collisions, and if
    /// not detected generate a `pub use inline_ns::*` or something like that.
    ///
    /// That's probably something we can do if we see this option is needed in a
    /// lot of cases, to improve it's usability, but my guess is that this is
    /// not going to be too useful.
    pub fn conservative_inline_namespaces(mut self) -> Builder {
        self.inputs.conservative_inline_namespaces = true;
        self
    }

    /// Whether inline functions should be generated or not.
    ///
    /// Note that they will usually not work. However you can use
    /// `-fkeep-inline-functions` or `-fno-inline-functions` if you are
    /// responsible of compiling the library to make them callable.
    pub fn generate_inline_functions(mut self, doit: bool) -> Self {
        self.inputs.generate_inline_functions = doit;
        self
    }

    /// Ignore functions.
    pub fn ignore_functions(mut self) -> Builder {
        self.inputs.codegen_config.remove(CodegenConfig::FUNCTIONS);
        self
    }

    /// Ignore methods.
    pub fn ignore_methods(mut self) -> Builder {
        self.inputs.codegen_config.remove(CodegenConfig::METHODS);
        self
    }

    /// Avoid generating any unstable Rust, such as Rust unions, in the generated bindings.
    #[deprecated(note = "please use `rust_target` instead")]
    pub fn unstable_rust(self, doit: bool) -> Self {
        let rust_target = if doit {
            RustTarget::Nightly
        } else {
            LATEST_STABLE_RUST
        };
        self.rust_target(rust_target)
    }

    /// Use core instead of libstd in the generated bindings.
    pub fn use_core(mut self) -> Builder {
        self.inputs.use_core = true;
        self
    }

    /// Use the given prefix for the raw types instead of `::std::os::raw`.
    pub fn ctypes_prefix<T: Into<String>>(mut self, prefix: T) -> Builder {
        self.inputs.ctypes_prefix = Some(prefix.into());
        self
    }

    /// Use the given prefix for the anon fields.
    pub fn anon_fields_prefix<T: Into<String>>(mut self, prefix: T) -> Builder {
        self.inputs.anon_fields_prefix = prefix.into();
        self
    }

    /// Allows configuring types in different situations, see the
    /// [`ParseCallbacks`](./callbacks/trait.ParseCallbacks.html) documentation.
    pub fn parse_callbacks(mut self, cb: Box<dyn ParseCallbacks>) -> Self {
        self.parse_callbacks = Some(cb);
        self
    }

    /// Choose what to generate using a
    /// [`CodegenConfig`](./struct.CodegenConfig.html).
    pub fn with_codegen_config(mut self, config: CodegenConfig) -> Self {
        self.inputs.codegen_config = config;
        self
    }

    /// Whether to detect include paths using clang_sys.
    pub fn detect_include_paths(mut self, doit: bool) -> Self {
        self.inputs.detect_include_paths = doit;
        self
    }

    /// Whether to try to fit macro constants to types smaller than u32/i32
    pub fn fit_macro_constants(mut self, doit: bool) -> Self {
        self.inputs.fit_macro_constants = doit;
        self
    }

    /// Prepend the enum name to constant or newtype variants.
    pub fn prepend_enum_name(mut self, doit: bool) -> Self {
        self.inputs.prepend_enum_name = doit;
        self
    }

    /// Set whether `size_t` should be translated to `usize` automatically.
    pub fn size_t_is_usize(mut self, is: bool) -> Self {
        self.inputs.size_t_is_usize = is;
        self
    }

    /// Set whether rustfmt should format the generated bindings.
    pub fn rustfmt_bindings(mut self, doit: bool) -> Self {
        self.inputs.rustfmt_bindings = doit;
        self
    }

    /// Set whether we should record matched items in our regex sets.
    pub fn record_matches(mut self, doit: bool) -> Self {
        self.inputs.record_matches = doit;
        self
    }

    /// Set the absolute path to the rustfmt configuration file, if None, the standard rustfmt
    /// options are used.
    pub fn rustfmt_configuration_file(mut self, path: Option<PathBuf>) -> Self {
        self = self.rustfmt_bindings(true);
        self.inputs.rustfmt_configuration_file = path;
        self
    }

    /// Sets an explicit path to rustfmt, to be used when rustfmt is enabled.
    pub fn with_rustfmt<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.inputs.rustfmt_path = Some(path.into());
        self
    }

    /// If true, always emit explicit padding fields.
    ///
    /// If a struct needs to be serialized in its native format (padding bytes
    /// and all), for example writing it to a file or sending it on the network,
    /// then this should be enabled, as anything reading the padding bytes of
    /// a struct may lead to Undefined Behavior.
    pub fn explicit_padding(mut self, doit: bool) -> Self {
        self.inputs.force_explicit_padding = doit;
        self
    }

    /// If true, enables experimental support to generate vtable functions.
    ///
    /// Should mostly work, though some edge cases are likely to be broken.
    pub fn vtable_generation(mut self, doit: bool) -> Self {
        self.inputs.vtable_generation = doit;
        self
    }

    /// If true, enables the sorting of the output in a predefined manner.
    ///
    /// TODO: Perhaps move the sorting order out into a config
    pub fn sort_semantically(mut self, doit: bool) -> Self {
        self.inputs.sort_semantically = doit;
        self
    }

    /// If true, merges extern blocks.
    pub fn merge_extern_blocks(mut self, doit: bool) -> Self {
        self.inputs.merge_extern_blocks = doit;
        self
    }

    /// Generate the Rust bindings using the options built up thus far.
    pub fn generate(self) -> Result<Bindings, BindgenError> {
        Bindings::generate(BindgenOptions {
            state: BindgenState::build(&self.inputs, self.parse_callbacks),
            inputs: self.inputs,
        })
    }

    /// Preprocess and dump the input header files to disk.
    ///
    /// This is useful when debugging bindgen, using C-Reduce, or when filing
    /// issues. The resulting file will be named something like `__bindgen.i` or
    /// `__bindgen.ii`
    pub fn dump_preprocessed_input(&self) -> io::Result<()> {
        let clang =
            clang_sys::support::Clang::find(None, &[]).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::Other,
                    "Cannot find clang executable",
                )
            })?;

        // The contents of a wrapper file that includes all the input header
        // files.
        let mut wrapper_contents = String::new();

        // Whether we are working with C or C++ inputs.
        let mut is_cpp = args_are_cpp(&self.inputs.clang_args);

        // For each input header, add `#include "$header"`.
        for header in &self.inputs.input_headers {
            is_cpp |= file_is_cpp(header);

            wrapper_contents.push_str("#include \"");
            wrapper_contents.push_str(header);
            wrapper_contents.push_str("\"\n");
        }

        // For each input header content, add a prefix line of `#line 0 "$name"`
        // followed by the contents.
        for &(ref name, ref contents) in &self.inputs.input_header_contents {
            is_cpp |= file_is_cpp(name);

            wrapper_contents.push_str("#line 0 \"");
            wrapper_contents.push_str(name);
            wrapper_contents.push_str("\"\n");
            wrapper_contents.push_str(contents);
        }

        let wrapper_path = PathBuf::from(if is_cpp {
            "__bindgen.cpp"
        } else {
            "__bindgen.c"
        });

        {
            let mut wrapper_file = File::create(&wrapper_path)?;
            wrapper_file.write_all(wrapper_contents.as_bytes())?;
        }

        let mut cmd = Command::new(&clang.path);
        cmd.arg("-save-temps")
            .arg("-E")
            .arg("-C")
            .arg("-c")
            .arg(&wrapper_path)
            .stdout(Stdio::piped());

        for a in &self.inputs.clang_args {
            cmd.arg(a);
        }

        for a in get_extra_clang_args() {
            cmd.arg(a);
        }

        let mut child = cmd.spawn()?;

        let mut preprocessed = child.stdout.take().unwrap();
        let mut file = File::create(if is_cpp {
            "__bindgen.ii"
        } else {
            "__bindgen.i"
        })?;
        io::copy(&mut preprocessed, &mut file)?;

        if child.wait()?.success() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "clang exited with non-zero status",
            ))
        }
    }

    /// Don't derive `PartialEq` for a given type. Regular
    /// expressions are supported.
    pub fn no_partialeq<T: Into<String>>(mut self, arg: T) -> Builder {
        self.inputs.no_partialeq_types.insert(arg.into());
        self
    }

    /// Don't derive `Copy` for a given type. Regular
    /// expressions are supported.
    pub fn no_copy<T: Into<String>>(mut self, arg: T) -> Self {
        self.inputs.no_copy_types.insert(arg.into());
        self
    }

    /// Don't derive `Debug` for a given type. Regular
    /// expressions are supported.
    pub fn no_debug<T: Into<String>>(mut self, arg: T) -> Self {
        self.inputs.no_debug_types.insert(arg.into());
        self
    }

    /// Don't derive/impl `Default` for a given type. Regular
    /// expressions are supported.
    pub fn no_default<T: Into<String>>(mut self, arg: T) -> Self {
        self.inputs.no_default_types.insert(arg.into());
        self
    }

    /// Don't derive `Hash` for a given type. Regular
    /// expressions are supported.
    pub fn no_hash<T: Into<String>>(mut self, arg: T) -> Builder {
        self.inputs.no_hash_types.insert(arg.into());
        self
    }

    /// Add `#[must_use]` for the given type. Regular
    /// expressions are supported.
    pub fn must_use_type<T: Into<String>>(mut self, arg: T) -> Builder {
        self.inputs.must_use_types.insert(arg.into());
        self
    }

    /// Set whether `arr[size]` should be treated as `*mut T` or `*mut [T; size]` (same for mut)
    pub fn array_pointers_in_arguments(mut self, doit: bool) -> Self {
        self.inputs.array_pointers_in_arguments = doit;
        self
    }

    /// Set the wasm import module name
    pub fn wasm_import_module_name<T: Into<String>>(
        mut self,
        import_name: T,
    ) -> Self {
        self.inputs.wasm_import_module_name = Some(import_name.into());
        self
    }

    /// Specify the dynamic library name if we are generating bindings for a shared library.
    pub fn dynamic_library_name<T: Into<String>>(
        mut self,
        dynamic_library_name: T,
    ) -> Self {
        self.inputs.dynamic_library_name = Some(dynamic_library_name.into());
        self
    }

    /// Require successful linkage for all routines in a shared library.
    /// This allows us to optimize function calls by being able to safely assume function pointers
    /// are valid.
    pub fn dynamic_link_require_all(mut self, req: bool) -> Self {
        self.inputs.dynamic_link_require_all = req;
        self
    }

    /// Generate bindings as `pub` only if the bound item is publically accessible by C++.
    pub fn respect_cxx_access_specs(mut self, doit: bool) -> Self {
        self.inputs.respect_cxx_access_specs = doit;
        self
    }

    /// Always translate enum integer types to native Rust integer types.
    ///
    /// This will result in enums having types such as `u32` and `i16` instead
    /// of `c_uint` and `c_short`. Types for Rustified enums are always
    /// translated.
    pub fn translate_enum_integer_types(mut self, doit: bool) -> Self {
        self.inputs.translate_enum_integer_types = doit;
        self
    }

    /// Generate types with C style naming.
    ///
    /// This will add prefixes to the generated type names. For example instead of a struct `A` we
    /// will generate struct `struct_A`. Currently applies to structs, unions, and enums.
    pub fn c_naming(mut self, doit: bool) -> Self {
        self.inputs.c_naming = doit;
        self
    }
}

#[derive(Debug)]
pub(crate) struct BindgenOptions {
    inputs: BindgenInputs,
    state: BindgenState,
}

impl BindgenOptions {
    pub(crate) fn inputs(&self) -> &BindgenInputs {
        &self.inputs
    }

    pub(crate) fn state(&self) -> &BindgenState {
        &self.state
    }

    pub(crate) fn state_mut(&mut self) -> &mut BindgenState {
        &mut self.state
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BindgenInputs {
    /// The set of types that have been blocklisted and should not appear
    /// anywhere in the generated code.
    blocklisted_types: RegexItems,

    /// The set of functions that have been blocklisted and should not appear
    /// in the generated code.
    blocklisted_functions: RegexItems,

    /// The set of items, regardless of item-type, that have been
    /// blocklisted and should not appear in the generated code.
    blocklisted_items: RegexItems,

    /// The set of files whose contents should be blocklisted and should not
    /// appear in the generated code.
    blocklisted_files: RegexItems,

    /// The set of types that should be treated as opaque structures in the
    /// generated code.
    opaque_types: RegexItems,

    /// The explicit rustfmt path.
    pub(crate) rustfmt_path: Option<PathBuf>,

    /// The path to which we should write a Makefile-syntax depfile (if any).
    pub(crate) depfile: Option<DepfileSpec>,

    /// The set of types that we should have bindings for in the generated
    /// code.
    ///
    /// This includes all types transitively reachable from any type in this
    /// set. One might think of allowlisted types/vars/functions as GC roots,
    /// and the generated Rust code as including everything that gets marked.
    allowlisted_types: RegexItems,

    /// Allowlisted functions. See docs for `allowlisted_types` for more.
    allowlisted_functions: RegexItems,

    /// Allowlisted variables. See docs for `allowlisted_types` for more.
    allowlisted_vars: RegexItems,

    /// The set of files whose contents should be allowlisted.
    allowlisted_files: RegexItems,

    /// The default style of code to generate for enums
    pub(crate) default_enum_style: EnumVariation,

    /// The enum patterns to mark an enum as a bitfield
    /// (newtype with bitwise operations).
    bitfield_enums: RegexItems,

    /// The enum patterns to mark an enum as a newtype.
    newtype_enums: RegexItems,

    /// The enum patterns to mark an enum as a global newtype.
    newtype_global_enums: RegexItems,

    /// The enum patterns to mark an enum as a Rust enum.
    rustified_enums: RegexItems,

    /// The enum patterns to mark an enum as a non-exhaustive Rust enum.
    rustified_non_exhaustive_enums: RegexItems,

    /// The enum patterns to mark an enum as a module of constants.
    constified_enum_modules: RegexItems,

    /// The enum patterns to mark an enum as a set of constants.
    constified_enums: RegexItems,

    /// The default type for C macro constants.
    pub(crate) default_macro_constant_type: MacroTypeVariation,

    /// The default style of code to generate for typedefs.
    pub(crate) default_alias_style: AliasVariation,

    /// Typedef patterns that will use regular type aliasing.
    type_alias: RegexItems,

    /// Typedef patterns that will be aliased by creating a new struct.
    new_type_alias: RegexItems,

    /// Typedef patterns that will be wrapped in a new struct and have
    /// Deref and Deref to their aliased type.
    new_type_alias_deref: RegexItems,

    /// The default style of code to generate for union containing non-Copy
    /// members.
    pub(crate) default_non_copy_union_style: crate::codegen::NonCopyUnionStyle,

    /// The union patterns to mark an non-Copy union as using the bindgen
    /// generated wrapper.
    pub(crate) bindgen_wrapper_union: RegexItems,

    /// The union patterns to mark an non-Copy union as using the
    /// `::core::mem::ManuallyDrop` wrapper.
    manually_drop_union: RegexItems,

    /// Whether we should generate builtins or not.
    pub(crate) builtins: bool,

    /// True if we should dump the Clang AST for debugging purposes.
    pub(crate) emit_ast: bool,

    /// True if we should dump our internal IR for debugging purposes.
    pub(crate) emit_ir: bool,

    /// Output graphviz dot file.
    pub(crate) emit_ir_graphviz: Option<String>,

    /// True if we should emulate C++ namespaces with Rust modules in the
    /// generated bindings.
    pub(crate) enable_cxx_namespaces: bool,

    /// True if we should try to find unexposed attributes in functions, in
    /// order to be able to generate #[must_use] attributes in Rust.
    pub(crate) enable_function_attribute_detection: bool,

    /// False if we should avoid mangling names with namespaces.
    pub(crate) enable_name_namespacing: bool,

    /// False if we should avoid generating nested struct names.
    pub(crate) enable_nested_struct_naming: bool,

    /// False if we should avoid embedding version identifiers into source code.
    pub(crate) enable_header_comment: bool,

    /// True if we should generate layout tests for generated structures.
    pub(crate) layout_tests: bool,

    /// True if we should implement the Debug trait for C/C++ structures and types
    /// that do not support automatically deriving Debug.
    pub(crate) impl_debug: bool,

    /// True if we should implement the PartialEq trait for C/C++ structures and types
    /// that do not support automatically deriving PartialEq.
    pub(crate) impl_partialeq: bool,

    /// True if we should derive Copy trait implementations for C/C++ structures
    /// and types.
    pub(crate) derive_copy: bool,

    /// True if we should derive Debug trait implementations for C/C++ structures
    /// and types.
    pub(crate) derive_debug: bool,

    /// True if we should derive Default trait implementations for C/C++ structures
    /// and types.
    pub(crate) derive_default: bool,

    /// True if we should derive Hash trait implementations for C/C++ structures
    /// and types.
    pub(crate) derive_hash: bool,

    /// True if we should derive PartialOrd trait implementations for C/C++ structures
    /// and types.
    pub(crate) derive_partialord: bool,

    /// True if we should derive Ord trait implementations for C/C++ structures
    /// and types.
    pub(crate) derive_ord: bool,

    /// True if we should derive PartialEq trait implementations for C/C++ structures
    /// and types.
    pub(crate) derive_partialeq: bool,

    /// True if we should derive Eq trait implementations for C/C++ structures
    /// and types.
    pub(crate) derive_eq: bool,

    /// True if we should avoid using libstd to use libcore instead.
    pub(crate) use_core: bool,

    /// An optional prefix for the "raw" types, like `c_int`, `c_void`...
    pub(crate) ctypes_prefix: Option<String>,

    /// The prefix for the anon fields.
    pub(crate) anon_fields_prefix: String,

    /// Whether to time the bindgen phases.
    pub(crate) time_phases: bool,

    /// Whether we should convert float types to f32/f64 types.
    pub(crate) convert_floats: bool,

    /// The set of raw lines to prepend to the top-level module of generated
    /// Rust code.
    pub(crate) raw_lines: Vec<String>,

    /// The set of raw lines to prepend to each of the modules.
    ///
    /// This only makes sense if the `enable_cxx_namespaces` option is set.
    pub(crate) module_lines: HashMap<String, Vec<String>>,

    /// The set of arguments to pass straight through to Clang.
    clang_args: Vec<String>,

    /// The input header files.
    input_headers: Vec<String>,

    /// Tuples of unsaved file contents of the form (name, contents).
    input_header_contents: Vec<(String, String)>,

    /// Which kind of items should we generate? By default, we'll generate all
    /// of them.
    pub(crate) codegen_config: CodegenConfig,

    /// Whether to treat inline namespaces conservatively.
    ///
    /// See the builder method description for more details.
    pub(crate) conservative_inline_namespaces: bool,

    /// Whether to keep documentation comments in the generated output. See the
    /// documentation for more details. Defaults to true.
    pub(crate) generate_comments: bool,

    /// Whether to generate inline functions. Defaults to false.
    pub(crate) generate_inline_functions: bool,

    /// Whether to allowlist types recursively. Defaults to true.
    pub(crate) allowlist_recursively: bool,

    /// Instead of emitting 'use objc;' to files generated from objective c files,
    /// generate '#[macro_use] extern crate objc;'
    pub(crate) objc_extern_crate: bool,

    /// Instead of emitting 'use block;' to files generated from objective c files,
    /// generate '#[macro_use] extern crate block;'
    pub(crate) generate_block: bool,

    /// Instead of emitting 'use block;' to files generated from objective c files,
    /// generate '#[macro_use] extern crate block;'
    pub(crate) block_extern_crate: bool,

    /// Whether to use the clang-provided name mangling. This is true and
    /// probably needed for C++ features.
    ///
    /// However, some old libclang versions seem to return incorrect results in
    /// some cases for non-mangled functions, see [1], so we allow disabling it.
    ///
    /// [1]: https://github.com/rust-lang/rust-bindgen/issues/528
    pub(crate) enable_mangling: bool,

    /// Whether to detect include paths using clang_sys.
    pub(crate) detect_include_paths: bool,

    /// Whether to try to fit macro constants into types smaller than u32/i32
    pub(crate) fit_macro_constants: bool,

    /// Whether to prepend the enum name to constant or newtype variants.
    pub(crate) prepend_enum_name: bool,

    /// Version of the Rust compiler to target
    rust_target: RustTarget,

    /// Features to enable, derived from `rust_target`
    rust_features: RustFeatures,

    /// Whether we should record which items in the regex sets ever matched.
    ///
    /// This may be a bit slower, but will enable reporting of unused allowlist
    /// items via the `error!` log.
    record_matches: bool,

    /// Whether `size_t` should be translated to `usize` automatically.
    pub(crate) size_t_is_usize: bool,

    /// Whether rustfmt should format the generated bindings.
    pub(crate) rustfmt_bindings: bool,

    /// The absolute path to the rustfmt configuration file, if None, the standard rustfmt
    /// options are used.
    pub(crate) rustfmt_configuration_file: Option<PathBuf>,

    /// The set of types that we should not derive `PartialEq` for.
    no_partialeq_types: RegexItems,

    /// The set of types that we should not derive `Copy` for.
    no_copy_types: RegexItems,

    /// The set of types that we should not derive `Debug` for.
    no_debug_types: RegexItems,

    /// The set of types that we should not derive/impl `Default` for.
    no_default_types: RegexItems,

    /// The set of types that we should not derive `Hash` for.
    no_hash_types: RegexItems,

    /// The set of types that we should be annotated with `#[must_use]`.
    must_use_types: RegexItems,

    /// Decide if C arrays should be regular pointers in rust or array pointers
    pub(crate) array_pointers_in_arguments: bool,

    /// Wasm import module name.
    pub(crate) wasm_import_module_name: Option<String>,

    /// The name of the dynamic library (if we are generating bindings for a shared library). If
    /// this is None, no dynamic bindings are created.
    pub(crate) dynamic_library_name: Option<String>,

    /// Require successful linkage for all routines in a shared library.
    /// This allows us to optimize function calls by being able to safely assume function pointers
    /// are valid. No effect if `dynamic_library_name` is None.
    pub(crate) dynamic_link_require_all: bool,

    /// Only make generated bindings `pub` if the items would be publically accessible
    /// by C++.
    pub(crate) respect_cxx_access_specs: bool,

    /// Always translate enum integer types to native Rust integer types.
    pub(crate) translate_enum_integer_types: bool,

    /// Generate types with C style naming.
    pub(crate) c_naming: bool,

    /// Always output explicit padding fields
    pub(crate) force_explicit_padding: bool,

    /// Emit vtable functions.
    pub(crate) vtable_generation: bool,

    /// Sort the code generation
    pub(crate) sort_semantically: bool,

    /// Deduplicate `extern` blocks.
    pub(crate) merge_extern_blocks: bool,
}

impl Default for BindgenInputs {
    fn default() -> Self {
        let rust_target = RustTarget::default();

        Self {
            rust_target,
            rust_features: rust_target.into(),
            blocklisted_types: Default::default(),
            blocklisted_functions: Default::default(),
            blocklisted_items: Default::default(),
            blocklisted_files: Default::default(),
            opaque_types: Default::default(),
            rustfmt_path: Default::default(),
            depfile: Default::default(),
            allowlisted_types: Default::default(),
            allowlisted_functions: Default::default(),
            allowlisted_vars: Default::default(),
            allowlisted_files: Default::default(),
            default_enum_style: Default::default(),
            bitfield_enums: Default::default(),
            newtype_enums: Default::default(),
            newtype_global_enums: Default::default(),
            rustified_enums: Default::default(),
            rustified_non_exhaustive_enums: Default::default(),
            constified_enums: Default::default(),
            constified_enum_modules: Default::default(),
            default_macro_constant_type: Default::default(),
            default_alias_style: Default::default(),
            type_alias: Default::default(),
            new_type_alias: Default::default(),
            new_type_alias_deref: Default::default(),
            default_non_copy_union_style: Default::default(),
            bindgen_wrapper_union: Default::default(),
            manually_drop_union: Default::default(),
            builtins: Default::default(),
            emit_ast: Default::default(),
            emit_ir: Default::default(),
            emit_ir_graphviz: Default::default(),
            layout_tests: true,
            impl_debug: Default::default(),
            impl_partialeq: Default::default(),
            derive_copy: true,
            derive_debug: true,
            derive_default: Default::default(),
            derive_hash: Default::default(),
            derive_partialord: Default::default(),
            derive_ord: Default::default(),
            derive_partialeq: Default::default(),
            derive_eq: Default::default(),
            enable_cxx_namespaces: Default::default(),
            enable_function_attribute_detection: Default::default(),
            enable_name_namespacing: true,
            enable_nested_struct_naming: true,
            enable_header_comment: true,
            use_core: Default::default(),
            ctypes_prefix: Default::default(),
            anon_fields_prefix: DEFAULT_ANON_FIELDS_PREFIX.into(),
            convert_floats: true,
            raw_lines: Default::default(),
            module_lines: Default::default(),
            clang_args: Default::default(),
            input_headers: Default::default(),
            input_header_contents: Default::default(),
            codegen_config: CodegenConfig::all(),
            conservative_inline_namespaces: Default::default(),
            generate_comments: true,
            generate_inline_functions: Default::default(),
            allowlist_recursively: true,
            generate_block: Default::default(),
            objc_extern_crate: Default::default(),
            block_extern_crate: Default::default(),
            enable_mangling: true,
            detect_include_paths: true,
            fit_macro_constants: Default::default(),
            prepend_enum_name: true,
            time_phases: Default::default(),
            record_matches: true,
            rustfmt_bindings: true,
            size_t_is_usize: Default::default(),
            rustfmt_configuration_file: Default::default(),
            no_partialeq_types: Default::default(),
            no_copy_types: Default::default(),
            no_debug_types: Default::default(),
            no_default_types: Default::default(),
            no_hash_types: Default::default(),
            must_use_types: Default::default(),
            array_pointers_in_arguments: Default::default(),
            wasm_import_module_name: Default::default(),
            dynamic_library_name: Default::default(),
            dynamic_link_require_all: Default::default(),
            respect_cxx_access_specs: Default::default(),
            translate_enum_integer_types: Default::default(),
            c_naming: Default::default(),
            force_explicit_padding: Default::default(),
            vtable_generation: Default::default(),
            sort_semantically: Default::default(),
            merge_extern_blocks: Default::default(),
        }
    }
}

impl BindgenInputs {
    /// Whether any of the enabled options requires `syn`.
    pub fn require_syn(&self) -> bool {
        self.sort_semantically || self.merge_extern_blocks
    }

    /// Update rust target version
    pub fn set_rust_target(&mut self, rust_target: RustTarget) {
        self.rust_target = rust_target;

        // Keep rust_features synced with rust_target
        self.rust_features = rust_target.into();
    }

    /// Get features supported by target Rust version
    pub fn rust_features(&self) -> RustFeatures {
        self.rust_features
    }
}

impl BindgenState {
    fn build(
        options: &BindgenInputs,
        parse_callbacks: Option<Box<dyn ParseCallbacks>>,
    ) -> Self {
        let mut clang_args = options.clang_args.clone();

        // Add any extra arguments from the environment to the clang command line.
        clang_args.extend(get_extra_clang_args());

        // Transform input headers to arguments on the clang command line.
        let (input_header, extra_input_headers) =
            if let Some((input_header, extra_input_headers)) =
                options.input_headers.split_last()
            {
                (Some(input_header.clone()), extra_input_headers.to_vec())
            } else {
                Default::default()
            };

        clang_args.extend(
            extra_input_headers
                .iter()
                .flat_map(|header| ["-include".into(), header.to_string()]),
        );

        let input_unsaved_files = options
            .input_header_contents
            .iter()
            .map(|(name, contents)| clang::UnsavedFile::new(&name, &contents))
            .collect();

        Self {
            allowlisted_vars: RegexSet::new(
                options.allowlisted_vars.clone(),
                options.record_matches,
            ),
            allowlisted_types: RegexSet::new(
                options.allowlisted_types.clone(),
                options.record_matches,
            ),
            allowlisted_functions: RegexSet::new(
                options.allowlisted_functions.clone(),
                options.record_matches,
            ),
            allowlisted_files: RegexSet::new(
                options.allowlisted_files.clone(),
                options.record_matches,
            ),
            blocklisted_types: RegexSet::new(
                options.blocklisted_types.clone(),
                options.record_matches,
            ),
            blocklisted_functions: RegexSet::new(
                options.blocklisted_functions.clone(),
                options.record_matches,
            ),
            blocklisted_items: RegexSet::new(
                options.blocklisted_items.clone(),
                options.record_matches,
            ),
            blocklisted_files: RegexSet::new(
                options.blocklisted_files.clone(),
                options.record_matches,
            ),
            opaque_types: RegexSet::new(
                options.opaque_types.clone(),
                options.record_matches,
            ),
            bitfield_enums: RegexSet::new(
                options.bitfield_enums.clone(),
                options.record_matches,
            ),
            constified_enums: RegexSet::new(
                options.constified_enums.clone(),
                options.record_matches,
            ),
            constified_enum_modules: RegexSet::new(
                options.constified_enum_modules.clone(),
                options.record_matches,
            ),
            newtype_enums: RegexSet::new(
                options.newtype_enums.clone(),
                options.record_matches,
            ),
            newtype_global_enums: RegexSet::new(
                options.newtype_global_enums.clone(),
                options.record_matches,
            ),
            rustified_enums: RegexSet::new(
                options.rustified_enums.clone(),
                options.record_matches,
            ),
            rustified_non_exhaustive_enums: RegexSet::new(
                options.rustified_non_exhaustive_enums.clone(),
                options.record_matches,
            ),
            type_alias: RegexSet::new(
                options.type_alias.clone(),
                options.record_matches,
            ),
            new_type_alias: RegexSet::new(
                options.new_type_alias.clone(),
                options.record_matches,
            ),
            new_type_alias_deref: RegexSet::new(
                options.new_type_alias_deref.clone(),
                options.record_matches,
            ),
            bindgen_wrapper_union: RegexSet::new(
                options.bindgen_wrapper_union.clone(),
                options.record_matches,
            ),
            manually_drop_union: RegexSet::new(
                options.manually_drop_union.clone(),
                options.record_matches,
            ),
            no_partialeq_types: RegexSet::new(
                options.no_partialeq_types.clone(),
                options.record_matches,
            ),
            no_copy_types: RegexSet::new(
                options.no_copy_types.clone(),
                options.record_matches,
            ),
            no_debug_types: RegexSet::new(
                options.no_debug_types.clone(),
                options.record_matches,
            ),
            no_default_types: RegexSet::new(
                options.no_default_types.clone(),
                options.record_matches,
            ),
            no_hash_types: RegexSet::new(
                options.no_hash_types.clone(),
                options.record_matches,
            ),
            must_use_types: RegexSet::new(
                options.must_use_types.clone(),
                options.record_matches,
            ),
            clang_args,
            input_header,
            extra_input_headers,
            input_unsaved_files,
            parse_callbacks,
        }
    }
}
