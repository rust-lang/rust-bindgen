//! Declarations and setter methods for `bindgen` options.
//!
//! The main entry point of this module is the [`options`] macro.
use crate::callbacks::ParseCallbacks;
use crate::codegen::{
    AliasVariation, EnumVariation, MacroTypeVariation, NonCopyUnionStyle,
};
use crate::deps::DepfileSpec;
use crate::features::{RustFeatures, RustTarget};
use crate::regex_set::RegexSet;
use crate::Abi;
use crate::Builder;
use crate::CodegenConfig;
use crate::FieldVisibilityKind;
use crate::Formatter;
use crate::HashMap;
use crate::DEFAULT_ANON_FIELDS_PREFIX;

use std::env;
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// Helper macro to set the default value of each option.
///
/// This macro is an internal implementation detail of the `options` macro and should not be used
/// directly.
macro_rules! default {
    () => {
        Default::default()
    };
    ($expr:expr) => {
        $expr
    };
}

/// Helper macro to set the conversion to CLI arguments for each option.
///
/// This macro is an internal implementation detail of the `options` macro and should not be used
/// directly.
macro_rules! as_args {
    ($flag:literal) => {
        |field, args| AsArgs::as_args(field, args, $flag)
    };
    ($expr:expr) => {
        $expr
    };
}

/// Helper function to ignore an option when converting it into CLI arguments.
///
/// This function is only used inside `options` and should not be used in other contexts.
fn ignore<T>(_: &T, _: &mut Vec<String>) {}

/// Helper function that appends extra documentation to [`Builder`] methods that support regular
/// expressions in their input. 
macro_rules! regex_option {
    ($(#[$attrs:meta])* pub fn $($tokens:tt)*) => {
        $(#[$attrs])*
        ///
        /// Regular expressions are supported. To match any items that start with `prefix` use the
        /// `"prefix.*"` regular expression.
        ///
        /// Check the [regular expression arguments](./struct.Builder.html#regular-expression-arguments)
        /// section and the [regex](https://docs.rs/regex) crate documentation for further
        /// information.
        pub fn $($tokens)*
    };
}

/// Macro used to generate the [`BindgenOptions`] type and the [`Builder`] setter methods for each
/// one of the fields of `BindgenOptions`.
///
/// The input format of this macro resembles a `struct` pattern. Each field of the `BindgenOptions`
/// type is declared using 4 items:
///
/// - `ty`: The type of the field.
/// - `default`: The default value for the field. If this item is omitted, `Default::default()` is
/// used instead, meaning that the type of the field must implement `Default`.
/// - `methods`: A block of code containing methods for the `Builder` type. These methods should be
/// related to the field being declared.
/// - `as_args`: This item declares how the field should be converted into a valid CLI argument for
/// `bindgen` and is used in the [`Builder::command_line_flags`] method which is used to do a
/// roundtrip test of the CLI args in the `bindgen-test` crate. This item can take one of the
/// following:
///   - A string literal with the flag if the type of the field implements the [`AsArgs`] trait.
///   - A closure with the signature `|field, args: &mut Vec<String>| -> ()` that pushes arguments
///   into the `args` buffer based on the value of the field. This is used if the field does not
///   implement `AsArgs` or if the implementation of the trait is not logically correct for the
///   option and a custom behavior must be taken into account.
///   - The `ignore` literal, which does not emit any CLI arguments for this field. This is useful
///   if the field cannot be used from the `bindgen` CLI.
/// ```
///
/// As an example, this would be the declaration of a `bool` field called `be_fun` whose default
/// value is `false` (the `Default` value for `bool`):
/// ```rust,ignore
/// be_fun: {
///    ty: bool,
///    methods: {
///        /// Ask bindgen to be fun. This option is disabled by default.
///        fn be_fun(mut self, doit: bool) -> Self {
///            self.options.be_fun = doit;
///            self
///        }
///    },
///    as_args: "--be-fun",
/// }
/// ```
///
/// However, we could also set the `be_fun` field to `true` by default and use a `--not-fun` flag
/// instead. This means that we have to add the `default` item and use a closure in the `as_args`
/// item:
/// ```rust,ignore
/// be_fun: {
///    ty: bool,
///    default: true,
///    methods: {
///        /// Ask bindgen to be fun. This option is enabled by default.
///        fn be_fun(mut self, doit: bool) -> Self {
///            self.options.be_fun = doit;
///            self
///        }
///    },
///    as_args: |be_fun, args| (!be_fun).as_args(args, "--not-fun"),
/// }
/// ```
/// More complex examples can be found in the sole invocation of this macro.
macro_rules! options {
    ($(
        $(#[doc = $docs:literal])+
        $field:ident: {
            ty: $ty:ty,
            $(default: $default:expr,)?
            methods: {$($methods_tokens:tt)*}$(,)?
            as_args: $as_args:expr$(,)?
        }$(,)?
    )*) => {
        #[derive(Debug, Clone)]
        pub(crate) struct BindgenOptions {
            $($(#[doc = $docs])* pub(crate) $field: $ty,)*
        }

        impl Default for BindgenOptions {
            fn default() -> Self {
                Self {
                    $($field: default!($($default)*),)*
                }
            }
        }

        impl Builder {
            /// Generates the command line flags used to create this [`Builder`].
            pub fn command_line_flags(&self) -> Vec<String> {
                let mut args = vec![];

                let headers = match self.options.input_headers.split_last() {
                    Some((header, headers)) => {
                        // The last input header is passed as an argument in the first position.
                        args.push(header.clone());
                        headers
                    },
                    None => &[]
                };

                $({
                    eprintln!("doing {}", stringify!($field));
                    let func: fn(&$ty, &mut Vec<String>) = as_args!($as_args);
                    func(&self.options.$field, &mut args);
                })*

                // Add the `--experimental` flag if `bindgen` is built with the `experimental`
                // feature.
                if cfg!(feature = "experimental") {
                    args.push("--experimental".to_owned());
                }

                // Add all the clang arguments.
                args.push("--".to_owned());

                if !self.options.clang_args.is_empty() {
                    args.extend_from_slice(&self.options.clang_args);
                }

                // We need to pass all but the last header via the `-include` clang argument.
                for header in headers {
                    args.push("-include".to_owned());
                    args.push(header.clone());
                }

                args
            }

            $($($methods_tokens)*)*
        }
    };
}

options! {
    /// The set of types that have been blocklisted and should not appear anywhere in the generated
    /// code.
    blocklisted_types: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Hide the given type from the generated bindings.
                pub fn blocklist_type<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.blocklisted_types.insert(arg);
                    self
                }
            }
        },
        as_args: "--blocklist-type",
    },
    /// The set of functions that have been blocklisted and should not appear in the generated
    /// code.
    blocklisted_functions: {
        ty: RegexSet,
        methods : {
            regex_option! {
                /// Hide the given function from the generated bindings.
                ///
                /// Methods can be blocklisted by prefixing the name of the type implementing them
                /// followed by an underscore. So if `Foo` has a method `bar`, it can be
                /// blocklisted as `Foo_bar`.
                pub fn blocklist_function<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.blocklisted_functions.insert(arg);
                    self
                }
            }
        },
        as_args: "--blocklist-function",
    },
    /// The set of items that have been blocklisted and should not appear in the generated code.
    blocklisted_items: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Hide the given item from the generated bindings, regardless of whether it is a
                /// type, function, module, etc.
                pub fn blocklist_item<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.blocklisted_items.insert(arg);
                    self
                }
            }
        },
        as_args: "--blocklist-item",
    },
    /// The set of files whose contents should be blocklisted and should not appear in the
    /// generated code.
    blocklisted_files: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Hide any contents of the given file from the generated bindings, regardless of
                /// whether the contents of the file are types, functions, modules etc.
                pub fn blocklist_file<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.blocklisted_files.insert(arg);
                    self
                }
            }
        },
        as_args: "--blocklist-file",
    },
    /// The set of types that should be treated as opaque structures in the generated code.
    opaque_types: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Treat the given type as opaque in the generated bindings.
                pub fn opaque_type<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.opaque_types.insert(arg);
                    self
                }
            }
        },
        as_args: "--opaque-type",
    },
    /// The explicit rustfmt path.
    rustfmt_path: {
        ty: Option<PathBuf>,
        methods: {
            /// Sets an explicit path to `rustfmt`.
            ///
            /// This option only makes sense if the [`Builder::formatter`] method is used with
            /// [`Formatter::Rustfmt`] or if the [`Builder::rustfmt_bindings`] method is used with
            /// `true`.
            pub fn with_rustfmt<P: Into<PathBuf>>(mut self, path: P) -> Self {
                self.options.rustfmt_path = Some(path.into());
                self
            }
        },
        // This option cannot be set from the CLI.
        as_args: ignore,
    },
    /// The path to which we should write a Makefile-syntax depfile (if any).
    depfile: {
        ty: Option<DepfileSpec>,
        methods: {
            /// Add a depfile output which will be written alongside the generated bindings.
            pub fn depfile<H: Into<String>, D: Into<PathBuf>>(
                mut self,
                output_module: H,
                depfile: D,
            ) -> Builder {
                self.options.depfile = Some(DepfileSpec {
                    output_module: output_module.into(),
                    depfile_path: depfile.into(),
                });
                self
            }
        },
        as_args: |depfile, args| {
            if let Some(depfile) = depfile {
                args.push("--depfile".into());
                args.push(depfile.depfile_path.display().to_string());
            }
        },
    },
    /// The set of types that we should have bindings for in the generated code.
    ///
    /// This includes all types transitively reachable from any type in this set. One might think
    /// of allowlisted types/vars/functions as GC roots, and the generated Rust code as including
    /// everything that gets marked.
    allowlisted_types: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Allowlist the given type so that it (and all types that it transitively refers
                /// to) appears in the generated bindings.
                pub fn allowlist_type<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.allowlisted_types.insert(arg);
                    self
                }
            }
        },
        as_args: "--allowlist-type",
    },
   /// Allowlisted functions. See docs for `allowlisted_types` for more.
    allowlisted_functions: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Allowlist the given function so that it (and all types that it transitively
                /// refers to) appears in the generated bindings.
                ///
                /// Methods can be allowlisted by prefixing the name of the type implementing them
                /// followed by an underscore. So if `Foo` has a method `bar`, it can be
                /// allowlisted as `Foo_bar`.
                pub fn allowlist_function<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.allowlisted_functions.insert(arg);
                    self
                }
            }
        },
        as_args: "--allowlist-function",
    },
    /// Allowlisted variables. See docs for `allowlisted_types` for more.
    allowlisted_vars: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Allowlist the given variable so that it (and all types that it transitively
                /// refers to) appears in the generated bindings.
                pub fn allowlist_var<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.allowlisted_vars.insert(arg);
                    self
                }
            }
        },
        as_args: "--allowlist-var",
    },
    /// The set of files whose contents should be allowlisted.
    allowlisted_files: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Allowlist the given file so that its contents appear in the generated bindings.
                pub fn allowlist_file<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.allowlisted_files.insert(arg);
                    self
                }
            }
        },
        as_args: "--allowlist-file",
    },
    /// The default style of code to generate for `enum`s.
    default_enum_style: {
        ty: EnumVariation,
        methods: {
            /// Set the default style of code to generate for `enum`s.
            ///
            /// If this method is not called the [`EnumVariation::Consts`] style will be used by
            /// default.
            ///
            /// To set the style for individual `enum`s, use [`Builder::bitfield_enum`],
            /// [`Builder::newtype_enum`], [`Builder::newtype_global_enum`],
            /// [`Builder::rustified_enums`], [`Builder::rustified_non_exhaustive_enums`],
            /// [`Builder::constified_enum_modules`] or [`Builder::constified_enums`].
            pub fn default_enum_style(
                mut self,
                arg: EnumVariation,
            ) -> Builder {
                self.options.default_enum_style = arg;
                self
            }
        },
        as_args: |variation, args| {
            if *variation != Default::default() {
                args.push("--default-enum-style".to_owned());
                args.push(variation.to_string());
            }
        },
    },
    /// The `enum` patterns to mark an `enum` as a bitfield (newtype with bitwise operations).
    bitfield_enums: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Mark the given `enum` as being bitfield-like.
                ///
                /// This is similar to the [`Builder::newtype_enum`] style, but with the bitwise
                /// operators implemented.
                pub fn bitfield_enum<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.bitfield_enums.insert(arg);
                    self
                }
            }
        },
        as_args: "--bitfield-enum",
    },
    /// The `enum` patterns to mark an `enum` as a newtype.
    newtype_enums: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Mark the given `enum`  as a newtype instead of a Rust `enum`.
                pub fn newtype_enum<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.newtype_enums.insert(arg);
                    self
                }
            }
        },
        as_args: "--newtype-enum",
    },
    /// The `enum` patterns to mark an `enum` as a global newtype.
    newtype_global_enums: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Mark the given `enum` as a newtype whose variants are exposed as global
                /// constants instead of a Rust `enum`.
                pub fn newtype_global_enum<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.newtype_global_enums.insert(arg);
                    self
                }
            }
        },
        as_args: "--newtype-global-enum",
    },
    /// The `enum` patterns to mark an `enum` as a Rust enum.
    rustified_enums: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Mark the given `enum` as a Rust `enum`. This makes `bindgen` generate `enum`
                /// variants instead of constants.
                ///
                /// **Use this with caution**, creating this in unsafe code (including FFI) with an
                /// invalid value will invoke undefined behaviour. To avoid this, use
                /// [`Builder::newtype_enum`] instead.
                pub fn rustified_enum<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.rustified_enums.insert(arg);
                    self
                }
            }
        },
        as_args: "--rustified-enum",
    },
    /// The `enum` patterns to mark an `enum` as a non-exhaustive Rust `enum`.
    rustified_non_exhaustive_enums: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Mark the given `enum` as a Rust `enum` with the `#[non_exhaustive]` attribute.
                ///
                /// This is similar to the [`Builder::rustified_enum`] style.
                pub fn rustified_non_exhaustive_enum<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.rustified_non_exhaustive_enums.insert(arg);
                    self
                }
            }
        },
        as_args: "--rustified-non-exhaustive-enums",
    },
    /// The `enum` patterns to mark an `enum` as a module of constants.
    constified_enum_modules: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Mark the given `enum` as a set of constants that should be put into a module.
                ///
                /// This makes `bindgen` generate modules containing constants instead of just
                /// constants.
                pub fn constified_enum_module<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.constified_enum_modules.insert(arg);
                    self
                }
            }
        },
        as_args: "--constified-enum-module",
    },
    /// The `enum` patterns to mark an `enum` as a set of constants.
    constified_enums: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Mark the given `enum` as a set of constants that are not to be put into a
                /// module.
                ///
                /// This is similar to the [`Builder::constified_enum_module`] style.
                pub fn constified_enum<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.constified_enums.insert(arg);
                    self
                }
            }
        },
        as_args: "--constified-enum",
    },
    /// The default type signedness for C macro constants.
    default_macro_constant_type: {
        ty: MacroTypeVariation,
        methods: {
            /// Set the default type signedness to be used for macro constants.
            ///
            /// If this method is not called, [`MacroTypeVariation::Unsigned`] is used by default.
            ///
            /// To set the type for individual macro constants, use [`ParseCallbacks::int_macro`].
            pub fn default_macro_constant_type(mut self, arg: MacroTypeVariation) -> Builder {
                self.options.default_macro_constant_type = arg;
                self
            }

        },
        as_args: |variation, args| {
            if *variation != Default::default() {
                args.push("--default-macro-constant-type".to_owned());
                args.push(variation.to_string());
            }
        },
    },
    /// The default style of code to generate for `typedef`s.
    default_alias_style: {
        ty: AliasVariation,
        methods: {
            /// Set the default style of code to generate for `typedef`s.
            ///
            /// If this method is not called, the [`AliasVaration::TypeAlias`] style is used by
            /// default.
            ///
            /// To set the style for individual `typedefs`s, use [`Builder::type_alias`],
            /// [`Builder::new_type_alias`] or [`Builder::new_type_alias_deref`].
            pub fn default_alias_style(
                mut self,
                arg: AliasVariation,
            ) -> Builder {
                self.options.default_alias_style = arg;
                self
            }
        },
        as_args: |variation, args| {
            if *variation != Default::default() {
                args.push("--default-alias-style".to_owned());
                args.push(variation.to_string());
            }
        },
    },
    /// `typedef` patterns that will use regular type aliasing.
    type_alias: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Mark the given `typedef` to use regular Rust type aliasing.
                ///
                /// This is the default behavior, meaning that this method should only be used if
                /// [`Builder::default_alias_style`] was called with a different style than
                /// [`AliasVariation::TypeAlias`].
                pub fn type_alias<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.type_alias.insert(arg);
                    self
                }
            }
        },
        as_args: "--type-alias",
    },
    /// `typedef` patterns that will be aliased by creating a new `struct`.
    new_type_alias: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Mark the given `typedef` to be generated as a new type by having the aliased
                /// type be wrapped in a `struct` with `#[repr(transparent)]`.
                ///
                /// This method can be used to enforce stricter type checking.
                pub fn new_type_alias<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.new_type_alias.insert(arg);
                    self
                }
            }
        },
        as_args: "--new-type-alias",
    },
    /// `typedef` patterns that will be wrapped in a new `struct` and implement `Deref` and
    /// `DerefMut`.
    new_type_alias_deref: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Mark the given `typedef` to be generated as a new type by having the aliased
                /// type be wrapped in a `struct` with `#[repr(transparent)]` that also implements
                /// `Deref` and `DerefMut` with the aliased type as a target.
                pub fn new_type_alias_deref<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.new_type_alias_deref.insert(arg);
                    self
                }
            }
        },
        as_args: "--new-type-alias-deref",
    },
    /// The default style of code to generate for `union`s containing non-`Copy` members.
    default_non_copy_union_style: {
        ty: NonCopyUnionStyle,
        methods: {
            /// Set the default style of code to generate for `union`s with a non-`Copy` member.
            ///
            /// To set the style for individual `union`s, use [`Builder::bindgen_wrapper_union`] or
            /// [`Builder::manually_drop_union`].
            pub fn default_non_copy_union_style(mut self, arg: NonCopyUnionStyle) -> Self {
                self.options.default_non_copy_union_style = arg;
                self
            }
        },
        as_args: |style, args| {
            if *style != Default::default() {
                args.push("--default-non-copy-union-style".to_owned());
                args.push(style.to_string());
            }
        },
    },
    /// The patterns marking non-`Copy` `union`s as using the `bindgen` generated wrapper.
    bindgen_wrapper_union: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Mark the given `union` to use a `bindgen`-generated wrapper for its members if at
                /// least one them is not `Copy`.
                pub fn bindgen_wrapper_union<T: AsRef<str>>(mut self, arg: T) -> Self {
                    self.options.bindgen_wrapper_union.insert(arg);
                    self
                }
            }
        },
        as_args: "--bindgen-wrapper-union",
    },
    /// The patterns marking non-`Copy` `union`s as using the `::core::mem::ManuallyDrop` wrapper.
    manually_drop_union: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Mark the given `union` to use [`::core::mem::ManuallyDrop`] for its members if
                /// at least one of them is not `Copy`.
                ///
                /// Note: `ManuallyDrop` was stabilized in Rust 1.20.0, do not use this option if
                /// your MSRV is lower.
                pub fn manually_drop_union<T: AsRef<str>>(mut self, arg: T) -> Self {
                    self.options.manually_drop_union.insert(arg);
                    self
                }
            }

        },
        as_args: "--manually-drop-union",
    },


    /// Whether we should generate builtins.
    builtins: {
        ty: bool,
        methods: {
            /// Emit bindings for built-in definitions (for example `__builtin_va_list`) in the
            /// generated Rust code.
            ///
            /// Bindings for built-in definitions are not emitted by default.
            pub fn emit_builtins(mut self) -> Builder {
                self.options.builtins = true;
                self
            }
        },
        as_args: "--builtins",
    },
    /// Whether we should dump the Clang AST for debugging purposes.
    emit_ast: {
        ty: bool,
        methods: {
            /// Emit the Clang AST to `stdout` for debugging purposes.
            ///
            /// This option is disabled by default.
            pub fn emit_clang_ast(mut self) -> Builder {
                self.options.emit_ast = true;
                self
            }
        },
        as_args: "--emit-clang-ast",
    },
    /// Whether we should dump our IR for debugging purposes.
    emit_ir: {
        ty: bool,
        methods: {
            /// Emit the `bindgen` internal representation to `stdout` for debugging purposes.
            ///
            /// This option is disabled by default.
            pub fn emit_ir(mut self) -> Builder {
                self.options.emit_ir = true;
                self
            }
        },
        as_args: "--emit-ir",
    },
    /// Output path for the graphviz dot file.
    emit_ir_graphviz: {
        ty: Option<String>,
        methods: {
            /// Set the path for the file where the`bindgen` internal representation will be
            /// emitted as a graph using the graphviz DOT language.
            ///
            /// This graph representation is not emitted by default.
            pub fn emit_ir_graphviz<T: Into<String>>(mut self, path: T) -> Builder {
                let path = path.into();
                self.options.emit_ir_graphviz = Some(path);
                self
            }
        },
        as_args: "--emit-ir-graphviz",
    },

    /// Whether we should emulate C++ namespaces with Rust modules.
    enable_cxx_namespaces: {
        ty: bool,
        methods: {
            /// Emulate C++ namespaces using Rust modules in the generated bindings.
            ///
            /// This option is not enabled by default.
            pub fn enable_cxx_namespaces(mut self) -> Builder {
                self.options.enable_cxx_namespaces = true;
                self
            }
        },
        as_args: "--enable-cxx-namespaces",
    },
    /// Whether we should try to find unexposed attributes in functions.
    enable_function_attribute_detection: {
        ty: bool,
        methods: {
            /// Enable detecting function attributes on C functions.
            ///
            /// This enables the following features:
            /// - Add `#[must_use]` attributes to Rust items whose C counterparts are marked as so.
            /// This feature also requires that the Rust target version supports these attributes.
            /// - Set `!` as the return type for Rust functions whose C counterparts are marked as
            /// diverging.
            ///
            /// This option can be quite slow in some cases (see [#1465]), so it is disabled by
            /// default.
            ///
            /// [#1465]: https://github.com/rust-lang/rust-bindgen/issues/1465
            pub fn enable_function_attribute_detection(mut self) -> Self {
                self.options.enable_function_attribute_detection = true;
                self
            }

        },
        as_args: "--enable-function-attribute-detection",
    },
    /// Whether we should avoid mangling names with namespaces.
    disable_name_namespacing: {
        ty: bool,
        methods: {
            /// Disable name auto-namespacing.
            ///
            /// By default, `bindgen` mangles names like `foo::bar::Baz` to look like `foo_bar_Baz`
            /// instead of just `Baz`. This method disables that behavior.
            ///
            /// Note that this intentionally does not change the names used for allowlisting and
            /// blocklisting, which should still be mangled with the namespaces. Additionally, this
            /// option may cause `bindgen` to generate duplicate names.
            pub fn disable_name_namespacing(mut self) -> Builder {
                self.options.disable_name_namespacing = true;
                self
            }
        },
        as_args: "--disable-name-namespacing",
    },
    /// Whether we should avoid generating nested `struct` names.
    disable_nested_struct_naming: {
        ty: bool,
        methods: {
            /// Disable nested `struct` naming.
            ///
            /// The following structs have different names for C and C++. In C, they are visible as
            /// `foo` and `bar`. In C++, they are visible as `foo` and `foo::bar`.
            ///
            /// ```c
            /// struct foo {
            ///     struct bar {
            ///     } b;
            /// };
            /// ```
            ///
            /// `bindgen` tries to avoid duplicate names by default so it follows the C++ naming
            /// convention and it generates `foo` and `foo_bar` instead of just `foo` and `bar`.
            ///
            /// This method disables this behavior and it is indented to be used only for headers
            /// that were written for C.
            pub fn disable_nested_struct_naming(mut self) -> Builder {
                self.options.disable_nested_struct_naming = true;
                self
            }
        },
        as_args: "--disable-nested-struct-naming",
    },
    /// Whether we should avoid embedding version identifiers into source code.
    disable_header_comment: {
        ty: bool,
        methods: {
            /// Do not insert the `bindgen` version identifier into the generated bindings.
            pub fn disable_header_comment(mut self) -> Self {
                self.options.disable_header_comment = true;
                self
            }

        },
        as_args: "--disable-header-comment",
    },
    /// Whether we should generate layout tests for generated `struct`s.
    layout_tests: {
        ty: bool,
        default: true,
        methods: {
            /// Set whether layout tests should be generated.
            ///
            /// Layout tests are generated by default.
            pub fn layout_tests(mut self, doit: bool) -> Self {
                self.options.layout_tests = doit;
                self
            }
        },
        as_args: |value, args| (!value).as_args(args, "--no-layout-tests"),
    },
    /// Whether we should implement `Debug` for types that cannot derive it.
    impl_debug: {
        ty: bool,
        methods: {
            /// Set whether `Debug` should be implemented for types that cannot derive it.
            ///
            /// This option is disabled by default.
            pub fn impl_debug(mut self, doit: bool) -> Self {
                self.options.impl_debug = doit;
                self
            }

        },
        as_args: "--impl-debug",
    },
    /// Whether we should implement `PartialEq` types that cannot derive it.
    impl_partialeq: {
        ty: bool,
        methods: {
            /// Set whether `PartialEq` should be implemented for types that cannot derive it.
            ///
            /// This option is disabled by default.
            pub fn impl_partialeq(mut self, doit: bool) -> Self {
                self.options.impl_partialeq = doit;
                self
            }
        },
        as_args: "--impl-partialeq",
    },
    /// Whether we should derive `Copy` when possible.
    derive_copy: {
        ty: bool,
        default: true,
        methods: {
            /// Set whether `Copy` should be derived by default.
            ///
            /// This option is enabled by default.
            pub fn derive_copy(mut self, doit: bool) -> Self {
                self.options.derive_copy = doit;
                self
            }
        },
        as_args: |value, args| (!value).as_args(args, "--no-derive-copy"),
    },

    /// Whether we should derive `Debug` when possible.
    derive_debug: {
        ty: bool,
        default: true,
        methods: {
            /// Set whether `Debug` should be derived by default.
            ///
            /// This option is enabled by default.
            pub fn derive_debug(mut self, doit: bool) -> Self {
                self.options.derive_debug = doit;
                self
            }
        },
        as_args: |value, args| (!value).as_args(args, "--no-derive-debug"),
    },

    /// Whether we should derive `Default` when possible.
    derive_default: {
        ty: bool,
        methods: {
            /// Set whether `Default` should be derived by default.
            ///
            /// This option is disabled by default.
            pub fn derive_default(mut self, doit: bool) -> Self {
                self.options.derive_default = doit;
                self
            }
        },
        as_args: |&value, args| {
            let arg = if value {
                "--with-derive-default"
            } else {
                "--no-derive-default"
            };

            args.push(arg.to_owned());
        },
    },
    /// Whether we should derive `Hash` when possible.
    derive_hash: {
        ty: bool,
        methods: {
            /// Set whether `Hash` should be derived by default.
            ///
            /// This option is disabled by default.
            pub fn derive_hash(mut self, doit: bool) -> Self {
                self.options.derive_hash = doit;
                self
            }
        },
        as_args: "--with-derive-hash",
    },
    /// Whether we should derive `PartialOrd` when possible.
    derive_partialord: {
        ty: bool,
        methods: {
            /// Set whether `PartialOrd` should be derived by default.
            ///
            /// Take into account that `Ord` cannot be derived for a type that does not implement
            /// `PartialOrd`. For this reason, setting this method to `false` also sets
            /// automatically [`Builder::derive_ord`] to `false`.
            ///
            /// This option is disabled by default.
            pub fn derive_partialord(mut self, doit: bool) -> Self {
                self.options.derive_partialord = doit;
                if !doit {
                    self.options.derive_ord = false;
                }
                self
            }
        },
        as_args: "--with-derive-partialord",
    },
    /// Whether we should derive `Ord` when possible.
    derive_ord: {
        ty: bool,
        methods: {
            /// Set whether `Ord` should be derived by default.
            ///
            /// Take into account that `Ord` cannot be derived for a type that does not implement
            /// `PartialOrd`. For this reason, the value set with this method will also be set
            /// automatically for [`Builder::derive_partialord`].
            ///
            /// This option is disabled by default.
            pub fn derive_ord(mut self, doit: bool) -> Self {
                self.options.derive_ord = doit;
                self.options.derive_partialord = doit;
                self
            }
        },
        as_args: "--with-derive-ord",
    },
    /// Whether we should derive `PartialEq` when possible.
    derive_partialeq: {
        ty: bool,
        methods: {
            /// Set whether `PartialEq` should be derived by default.
            ///
            /// Take into account that `Eq` cannot be derived for a type that does not implement
            /// `PartialEq`. For this reason, setting this method to `false` also sets
            /// automatically [`Builder::derive_eq`] to `false`.
            ///
            /// This option is disabled by default.
            pub fn derive_partialeq(mut self, doit: bool) -> Self {
                self.options.derive_partialeq = doit;
                if !doit {
                    self.options.derive_eq = false;
                }
                self
            }
        },
        as_args: "--with-derive-partialeq",
    },
    /// Whether we should derive `Eq` when possible.
    derive_eq: {
        ty: bool,
        methods: {
            /// Set whether `Eq` should be derived by default.
            ///
            /// Take into account that `Eq` cannot be derived for a type that does not implement
            /// `PartialEq`. For this reason, the value set with this method will also be set
            /// automatically for [`Builder::derive_partialeq`].
            ///
            /// This option is disabled by default.
            pub fn derive_eq(mut self, doit: bool) -> Self {
                self.options.derive_eq = doit;
                if doit {
                    self.options.derive_partialeq = doit;
                }
                self
            }
        },
        as_args: "--with-derive-eq",
    },
    /// Whether we should use `core` instead of `std`.
    ///
    /// If this option is enabled and the Rust target version is greater than 1.64, the prefix for
    /// C platform-specific types will be `::core::ffi` instead of `::core::os::raw`.
    use_core: {
        ty: bool,
        methods: {
            /// Use `core` instead of `std` in the generated bindings.
            ///
            /// This option is disabled by default.
            pub fn use_core(mut self) -> Builder {
                self.options.use_core = true;
                self
            }

        },
        as_args: "--use-core",
    },
    /// An optional prefix for the C platform-specific types.
    ctypes_prefix: {
        ty: Option<String>,
        methods: {
            /// Use the given prefix for the raw types instead of `::std::os::raw`.
            ///
            /// To use `::core::ffi` or `::core::os::raw` see [`Bulider::use_core`].
            pub fn ctypes_prefix<T: Into<String>>(mut self, prefix: T) -> Builder {
                self.options.ctypes_prefix = Some(prefix.into());
                self
            }
        },
        as_args: "--ctypes-prefix",
    },
    /// The prefix for anonymous fields.
    anon_fields_prefix: {
        ty: String,
        default: DEFAULT_ANON_FIELDS_PREFIX.into(),
        methods: {
            /// Use the given prefix for the anonymous fields.
            ///
            /// The default prefix is `__bindgen_anon_`.")]
            ///
            /// An anonymous field, is a field of a C/C++ type that does not have a name. For
            /// example, in the following C code:
            /// ```c
            /// struct integer {
            ///   struct {
            ///     int inner;
            ///   };
            /// }
            /// ```
            ///
            /// The only field of the `integer` `struct` is an anonymous field and its Rust
            /// representation will be named using this prefix followed by an integer identifier.
            pub fn anon_fields_prefix<T: Into<String>>(mut self, prefix: T) -> Builder {
                self.options.anon_fields_prefix = prefix.into();
                self
            }
        },
        as_args: |prefix, args| {
            if prefix != DEFAULT_ANON_FIELDS_PREFIX {
                args.push("--anon-fields-prefix".to_owned());
                args.push(prefix.clone());
            }
        },
    },
    /// Whether to measure the time for each one of the `bindgen` phases.
    time_phases: {
        ty: bool,
        methods: {
            /// Set whether to measure the elapsed time for each one of the `bindgen` phases. This
            /// information is printed to `stderr`.
            ///
            /// This option is disabled by default.
            pub fn time_phases(mut self, doit: bool) -> Self {
                self.options.time_phases = doit;
                self
            }
        },
        as_args: "--time-phases",
    },
    /// Whether to convert C float types to `f32` and `f64`.
    convert_floats: {
        ty: bool,
        default: true,
        methods: {
            /// Avoid converting C float types to `f32` and `f64` by default.
            pub fn no_convert_floats(mut self) -> Self {
                self.options.convert_floats = false;
                self
            }
        },
        as_args: |value, args| (!value).as_args(args, "--no-convert-floats"),
    },
    /// The set of raw lines to be prepended to the top-level module of the generated Rust code.
    raw_lines: {
        ty: Vec<String>,
        methods: {
            /// Add a line of Rust code at the beginning of the generated bindings. The string is
            /// passed through without any modification.
            pub fn raw_line<T: Into<String>>(mut self, arg: T) -> Self {
                self.options.raw_lines.push(arg.into());
                self
            }
        },
        as_args: |raw_lines, args| {
            for line in raw_lines {
                args.push("--raw-line".to_owned());
                args.push(line.clone());
            }
        },
    },
    /// The set of raw lines to prepend to different modules.
    module_lines: {
        ty: HashMap<String, Vec<String>>,
        methods: {
            /// Add a given line to the beginning of a given module.
            ///
            /// This option only makes sense if the [`Builder::enable_cxx_namespaces`] option is
            /// enabled.
            pub fn module_raw_line<T, U>(mut self, module: T, line: U) -> Self
            where
                T: Into<String>,
                U: Into<String>,
            {
                self.options
                    .module_lines
                    .entry(module.into())
                    .or_insert_with(Vec::new)
                    .push(line.into());
                self
            }
        },
        as_args: |module_lines, args| {
            for (module, lines) in module_lines {
                for line in lines.iter() {
                    args.push("--module-raw-line".to_owned());
                    args.push(module.clone());
                    args.push(line.clone());
                }
            }
        },
    },
    /// The input header files.
    input_headers: {
        ty:  Vec<String>,
        methods: {
            /// Add an input C/C++ header to generate bindings for.
            ///
            /// This can be used to generate bindings for a single header:
            ///
            /// ```ignore
            /// let bindings = bindgen::Builder::default()
            ///     .header("input.h")
            ///     .generate()
            ///     .unwrap();
            /// ```
            ///
            /// Or for multiple headers:
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
                self.options.input_headers.push(header.into());
                self
            }
        },
        // This field is handled specially inside the macro.
        as_args: ignore,
    },
    /// The set of arguments to be passed straight through to Clang.
    clang_args: {
        ty: Vec<String>,
        methods: {
            /// Add an argument to be passed straight through to Clang.
            pub fn clang_arg<T: Into<String>>(self, arg: T) -> Builder {
                self.clang_args([arg.into()])
            }

            /// Add several arguments to be passed straight through to Clang.
            pub fn clang_args<I: IntoIterator>(mut self, args: I) -> Builder
            where
                I::Item: AsRef<str>,
            {
                for arg in args {
                    self.options.clang_args.push(arg.as_ref().to_owned());
                }
                self
            }
        },
        // This field is handled specially inside the macro.
        as_args: ignore,
    },
    /// Tuples of unsaved file contents of the form (name, contents).
    input_header_contents: {
        ty: Vec<(String, String)>,
        methods: {
            /// Add `contents` as an input C/C++ header named `name`.
            ///
            /// This can be used to inject additional C/C++ code as an input without having to
            /// create a header file.
            pub fn header_contents(mut self, name: &str, contents: &str) -> Builder {
                // Apparently clang relies on having virtual FS correspondent to
                // the real one, so we need absolute paths here
                let absolute_path = env::current_dir()
                    .expect("Cannot retrieve current directory")
                    .join(name)
                    .to_str()
                    .expect("Cannot convert current directory name to string")
                    .to_owned();
                self.options
                    .input_header_contents
                    .push((absolute_path, contents.into()));
                self
            }
        },
        // Header contents cannot be added from the CLI.
        as_args: ignore,
    },
    /// A user-provided visitor to allow customizing different kinds of situations.
    parse_callbacks: {
        ty: Vec<Rc<dyn ParseCallbacks>>,
        methods: {
            /// Add a new [`ParseCallbacks`] instance to configure types in different situations.
            pub fn parse_callbacks(mut self, cb: Box<dyn ParseCallbacks>) -> Self {
                self.options.parse_callbacks.push(Rc::from(cb));
                self
            }
        },
        as_args: |parse_callbacks, args| {
            #[cfg(feature = "__cli")]
            for callbacks in parse_callbacks {
                args.extend(callbacks.cli_args());
            }
        },
    },
    /// Which kind of items should we generate. We generate all of them by default.
    codegen_config: {
        ty: CodegenConfig,
        default: CodegenConfig::all(),
        methods: {
            /// Do not generate any functions.
            ///
            /// This option is disabled by default.
            pub fn ignore_functions(mut self) -> Builder {
                self.options.codegen_config.remove(CodegenConfig::FUNCTIONS);
                self
            }

            /// Do not generate any methods.
            ///
            /// This option is disabled by default.
            pub fn ignore_methods(mut self) -> Builder {
                self.options.codegen_config.remove(CodegenConfig::METHODS);
                self
            }

            /// Choose what to generate using a [`CodegenConfig`].
            ///
            /// This option overlaps with [`Builder::ignore_functions`] and
            /// [`Builder::ignore_methods`].
            ///
            /// All the items in `CodegenConfig` are generated by default.
            pub fn with_codegen_config(mut self, config: CodegenConfig) -> Self {
                self.options.codegen_config = config;
                self
            }
        },
        as_args: |codegen_config, args| {
            if !codegen_config.functions() {
                args.push("--ignore-functions".to_owned());
            }

            args.push("--generate".to_owned());

            //Temporary placeholder for below 4 options
            let mut options: Vec<String> = Vec::new();
            if codegen_config.functions() {
                options.push("functions".to_owned());
            }

            if codegen_config.types() {
                options.push("types".to_owned());
            }

            if codegen_config.vars() {
                options.push("vars".to_owned());
            }

            if codegen_config.methods() {
                options.push("methods".to_owned());
            }

            if codegen_config.constructors() {
                options.push("constructors".to_owned());
            }

            if codegen_config.destructors() {
                options.push("destructors".to_owned());
            }

            args.push(options.join(","));

            if !codegen_config.methods() {
                args.push("--ignore-methods".to_owned());
            }
        },
    },
    /// Whether to treat inline namespaces conservatively.
    conservative_inline_namespaces: {
        ty: bool,
        methods: {
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
            /// to make the rest of `bindgen` users pay an usability penalty for that.
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
            ///
            /// This option is disabled by default.
            pub fn conservative_inline_namespaces(mut self) -> Builder {
                self.options.conservative_inline_namespaces = true;
                self
            }
        },
        as_args: "--conservative-inline-namespaces",
    },
    /// Whether to keep documentation comments in the generated output.
    generate_comments: {
        ty: bool,
        default: true,
        methods: {
            /// Set whether the generated bindings should contain documentation comments (docstrings)
            ///.
            ///
            /// This option is enabled by default.
            ///
            /// Note that clang by default excludes comments from system headers, pass
            /// `-fretain-comments-from-system-headers` as [`clang_arg`][Builder::clang_arg] to
            /// include them. It can also be told to process all comments (not just documentation
            /// ones) using the `-fparse-all-comments` flag. See [slides on clang comment parsing](
            /// https://llvm.org/devmtg/2012-11/Gribenko_CommentParsing.pdf) for background and
            /// examples.
            pub fn generate_comments(mut self, doit: bool) -> Self {
                self.options.generate_comments = doit;
                self
            }
        },
        as_args: |value, args| (!value).as_args(args, "--no-doc-comments"),
    },
    /// Whether to generate inline functions.
    generate_inline_functions: {
        ty: bool,
        methods: {
            /// Set whether to generate inline functions.
            ///
            /// This option is disabled by default.
            ///
            /// Note that they will usually not work. However you can use `-fkeep-inline-functions`
            /// or `-fno-inline-functions` if you are responsible of compiling the library to make
            /// them callable. See [`Builder::wrap_static_fns`] for an alternative.
            pub fn generate_inline_functions(mut self, doit: bool) -> Self {
                self.options.generate_inline_functions = doit;
                self
            }
        },
        as_args: "--generate-inline-functions",
    },
    /// Whether to allowlist types recursively.
    allowlist_recursively: {
        ty: bool,
        default: true,
        methods: {
            /// Set whether to allowlist recursively.
            ///
            /// This option is enabled by default.
            ///
            /// Given that we have explicitly allowlisted the "initiate_dance_party" function in
            /// this C header:
            ///
            /// ```c
            /// typedef struct MoonBoots {
            ///     int bouncy_level;
            /// } MoonBoots;
            ///
            /// void initiate_dance_party(MoonBoots* boots);
            /// ```
            ///
            /// We would normally generate bindings to both the `initiate_dance_party` function and
            /// the `MoonBoots` struct that it transitively references. By configuring with
            /// `allowlist_recursively(false)`, `bindgen` will not emit bindings for anything
            /// except the explicitly allowlisted items, and there would be no emitted struct
            /// definition for `MoonBoots`. However, the `initiate_dance_party` function would
            /// still reference `MoonBoots`!
            ///
            /// **Disabling this feature will almost certainly cause `bindgen` to emit bindings
            /// that will not compile!** If you disable this feature, then it is *your*
            /// responsibility to provide definitions for every type that is referenced from an
            /// explicitly allowlisted item. One way to provide the definitions is by using the
            /// [`Builder::raw_line`] method, another would be to define them in Rust and then
            /// `include!(...)` the bindings immediately afterwards.
            pub fn allowlist_recursively(mut self, doit: bool) -> Self {
                self.options.allowlist_recursively = doit;
                self
            }
        },
        as_args: |value, args| (!value).as_args(args, "--no-recursive-allowlist"),
    },
    /// Instead of emitting 'use objc;' to files generated from objective c files,
    /// generate '#[macro_use] extern crate objc;'
    objc_extern_crate: {
        ty: bool,
        methods: {
            /// Emit `#[macro_use] extern crate objc;` instead of `use objc;` in the prologue of
            /// the files generated from objective-c files.
            ///
            /// This option is disabled by default.
            pub fn objc_extern_crate(mut self, doit: bool) -> Self {
                self.options.objc_extern_crate = doit;
                self
            }
        },
        as_args: "--objc-extern-crate",
    },
    /// Whether to generate proper block signatures instead of void pointers.
    generate_block: {
        ty: bool,
        methods: {
            /// Generate proper block signatures instead of void pointers.
            ///
            /// This option is disabled by default.
            pub fn generate_block(mut self, doit: bool) -> Self {
                self.options.generate_block = doit;
                self
            }
        },
        as_args: "--generate-block",
    },
    /// Instead of emitting 'use block;' to files generated from objective c files,
    /// generate '#[macro_use] extern crate block;'
    block_extern_crate: {
        ty: bool,
        methods: {
            /// Generate `#[macro_use] extern crate block;` instead of `use block;`
            /// in the prologue of the files generated from apple block files.
            ///
            /// This option is disabled by default.
            pub fn block_extern_crate(mut self, doit: bool) -> Self {
                self.options.block_extern_crate = doit;
                self
            }
        },
        as_args: "--block-extern-crate",
    },
    /// Whether to use the clang-provided name mangling.
    enable_mangling: {
        ty: bool,
        default: true,
        methods: {
            /// Whether to use the clang-provided name mangling. This is probably needed for C++ features.
            ///
            /// This option is enabled by default.
            ///
            /// We allow disabling this option because some old `libclang` versions seem to return
            /// incorrect results in some cases for non-mangled functions, see [#528].
            ///
            /// [#528]: https://github.com/rust-lang/rust-bindgen/issues/528
            pub fn trust_clang_mangling(mut self, doit: bool) -> Self {
                self.options.enable_mangling = doit;
                self
            }

        },
        as_args: |value, args| (!value).as_args(args, "--distrust-clang-mangling"),
    },
    /// Whether to detect include paths using `clang_sys`.
    detect_include_paths: {
        ty: bool,
        default: true,
        methods: {
            /// Whether to detect include paths using `clang_sys`.
            ///
            /// This option is enabled by default.
            pub fn detect_include_paths(mut self, doit: bool) -> Self {
                self.options.detect_include_paths = doit;
                self
            }
        },
        as_args: |value, args| (!value).as_args(args, "--no-include-path-detection"),
    },
    /// Whether to try to fit macro constants into types smaller than `u32` and `i32`.
    fit_macro_constants: {
        ty: bool,
        methods: {
            /// Whether to try to fit macro constants to types smaller than `u32` and `i32`.
            ///
            /// This option is disabled by default.
            pub fn fit_macro_constants(mut self, doit: bool) -> Self {
                self.options.fit_macro_constants = doit;
                self
            }
        },
        as_args: "--fit-macro-constant-types",
    },
    /// Whether to prepend the enum name to constant or newtype variants.
    prepend_enum_name: {
        ty: bool,
        default: true,
        methods: {
            /// Prepend the enum name to constant or newtype variants.
            ///
            /// This option is enabled by default.
            pub fn prepend_enum_name(mut self, doit: bool) -> Self {
                self.options.prepend_enum_name = doit;
                self
            }
        },
        as_args: |value, args| (!value).as_args(args, "--no-prepend-enum-name"),
    },
    /// Version of the Rust compiler to target.
    rust_target: {
        ty: RustTarget,
        methods: {
            /// Specify the Rust target version.
            ///
            /// The default target is the latest stable Rust version.
            pub fn rust_target(mut self, rust_target: RustTarget) -> Self {
                #[allow(deprecated)]
                if rust_target <= RustTarget::Stable_1_30 {
                    warn!(
                        "The {} rust target is deprecated. If you have a good reason to use this target please report it at https://github.com/rust-lang/rust-bindgen/issues",
                        String::from(rust_target)
                    );
                }
                self.options.set_rust_target(rust_target);
                self
            }
        },
        as_args: |rust_target, args| {
            args.push("--rust-target".to_owned());
            args.push((*rust_target).into());
        },
    },
    /// Features to be enabled. They are derived from `rust_target`.
    rust_features: {
        ty: RustFeatures,
        default: RustTarget::default().into(),
        methods: {
            /// Disable support for native Rust unions, if supported.
            ///
            /// The default value of this option is set based on the value passed to
            /// [`Builder::rust_target`].
            pub fn disable_untagged_union(mut self) -> Self {
                self.options.rust_features.untagged_union = false;
                self
            }
        },
        as_args: |features, args| {
            // FIXME(emilio): This is a bit hacky, maybe we should stop re-using the
            // RustFeatures to store the "disable_untagged_union" call, and make it
            // a different flag that we check elsewhere / in generate().
            if !features.untagged_union {
                args.push("--disable-untagged-union".to_owned());
            }
        },
    },
    /// Whether we should record which items in the regex sets did match any C items.
    record_matches: {
        ty: bool,
        default: true,
        methods: {
            /// Set whether we should record which items in our regex sets did match any C items.
            ///
            /// This option is enabled by default
            pub fn record_matches(mut self, doit: bool) -> Self {
                self.options.record_matches = doit;
                self
            }

        },
        as_args: |value, args| (!value).as_args(args, "--no-record-matches"),
    },
    /// Whether `size_t` should be translated to `usize` automatically.
    size_t_is_usize: {
        ty: bool,
        default: true,
        methods: {
            /// Set whether `size_t` should be translated to `usize` automatically.
            ///
            /// This option is enabled by default
            pub fn size_t_is_usize(mut self, is: bool) -> Self {
                self.options.size_t_is_usize = is;
                self
            }
        },
        as_args: |value, args| (!value).as_args(args, "--no-size_t-is-usize"),
    },
    /// The tool that should be used to format the generated bindings.
    formatter: {
        ty: Formatter,
        methods: {
            #[cfg_attr(feature = "prettyplease", deprecated)]
            /// Set whether rustfmt should format the generated bindings.
            ///
            /// This option is enabled by default.
            ///
            /// This method overlaps in functionality with the more general [`Builder::formatter`].
            /// Thus, the latter should be prefered.
            pub fn rustfmt_bindings(mut self, doit: bool) -> Self {
                self.options.formatter = if doit {
                    Formatter::Rustfmt
                } else {
                    Formatter::None
                };
                self
            }

            /// Set which tool should be used to format the generated bindings.
            ///
            /// The default formatter is [`Formatter::Rustfmt`].
            ///
            /// To be able to choose `prettyplease` as a formatter, the `"prettyplease"` feature
            /// for `bindgen` must be enabled in the Cargo manifest.
            pub fn formatter(mut self, formatter: Formatter) -> Self {
                self.options.formatter = formatter;
                self
            }
        },
        as_args: |formatter, args| {
            if *formatter != Default::default() {
                args.push("--formatter".to_owned());
                args.push(formatter.to_string());
            }
        },
    },
    /// The absolute path to the rustfmt configuration file.
    rustfmt_configuration_file: {
        ty: Option<PathBuf>,
        methods: {
            /// Set the absolute path to the rustfmt configuration file.
            ///
            /// The default `rustfmt` options are used if this method is called with `None` or if
            /// this method is not called at all.
            ///
            /// Calling this method will set the [`Bindings::rustfmt_bindings`] option to `true`
            /// and the [`Bindings::formatter`] option to [`Formatter::Rustfmt`].
            pub fn rustfmt_configuration_file(mut self, path: Option<PathBuf>) -> Self {
                self = self.formatter(Formatter::Rustfmt);
                self.options.rustfmt_configuration_file = path;
                self
            }
        },
        as_args: "--rustfmt-configuration-file",
    },
    /// The set of types that should not derive `PartialEq`.
    no_partialeq_types: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Do not derive `PartialEq` for a given type.
                pub fn no_partialeq<T: Into<String>>(mut self, arg: T) -> Builder {
                    self.options.no_partialeq_types.insert(arg.into());
                    self
                }
            }
        },
        as_args: "--no-partialeq",
    },
    /// The set of types that should not derive `Copy`.
    no_copy_types: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Do not derive `Copy` and `Clone` for a given type.
                pub fn no_copy<T: Into<String>>(mut self, arg: T) -> Self {
                    self.options.no_copy_types.insert(arg.into());
                    self
                }
            }
        },
        as_args: "--no-copy",
    },
    /// The set of types that should not derive `Debug`.
    no_debug_types: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Do not derive `Debug` for a given type.
                pub fn no_debug<T: Into<String>>(mut self, arg: T) -> Self {
                    self.options.no_debug_types.insert(arg.into());
                    self
                }
            }
        },
        as_args: "--no-debug",
    },
    /// The set of types that should not derive or implement `Default`.
    no_default_types: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Do not derive or implement `Default` for a given type.
                pub fn no_default<T: Into<String>>(mut self, arg: T) -> Self {
                    self.options.no_default_types.insert(arg.into());
                    self
                }
            }
        },
        as_args: "--no-default",
    },
    /// The set of types that should not derive `Hash`.
    no_hash_types: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Do not derive `Hash` for a given type.
                pub fn no_hash<T: Into<String>>(mut self, arg: T) -> Builder {
                    self.options.no_hash_types.insert(arg.into());
                    self
                }
            }
        },
        as_args: "--no-hash",
    },
    /// The set of types that should be annotated with `#[must_use]`.
    must_use_types: {
        ty: RegexSet,
        methods: {
            regex_option! {
                /// Annotate the given type with the `#[must_use]` attribute.
                pub fn must_use_type<T: Into<String>>(mut self, arg: T) -> Builder {
                    self.options.must_use_types.insert(arg.into());
                    self
                }
            }
        },
        as_args: "--must-use-type",
    },
    /// Decide if C arrays should be regular pointers in rust or array pointers
    array_pointers_in_arguments: {
        ty: bool,
        methods: {
            /// Translate arrays `T arr[size]` into array pointers `*mut [T; size]` instead of
            /// translating them as `*mut T` which is the default. The same is done for `*const`
            /// pointers.
            pub fn array_pointers_in_arguments(mut self, doit: bool) -> Self {
                self.options.array_pointers_in_arguments = doit;
                self
            }

        },
        as_args: "--use-array-pointers-in-arguments",
    },
    /// Wasm import module name.
    wasm_import_module_name: {
        ty: Option<String>,
        methods: {
            /// Adds the `#[link(wasm_import_module = import_name)]` attribute to all the `extern`
            /// blocks generated by `bindgen`.
            ///
            /// This option is disabled by default.
            pub fn wasm_import_module_name<T: Into<String>>(
                mut self,
                import_name: T,
            ) -> Self {
                self.options.wasm_import_module_name = Some(import_name.into());
                self
            }
        },
        as_args: "--wasm-import-module-name",
    },
    /// The name of the dynamic library (if we are generating bindings for a shared library).
    dynamic_library_name: {
        ty: Option<String>,
        methods: {
            /// Generate bindings for a shared library with the given name.
            ///
            /// This option is disabled by default.
            pub fn dynamic_library_name<T: Into<String>>(
                mut self,
                dynamic_library_name: T,
            ) -> Self {
                self.options.dynamic_library_name = Some(dynamic_library_name.into());
                self
            }
        },
        as_args: "--dynamic-loading",
    },
    /// Require successful linkage for all routines in a shared library.
    dynamic_link_require_all: {
        ty: bool,
        methods: {
            /// Require successful linkage for all routines in a shared library. This allows us to
            /// optimize function calls by being able to safely assume function pointers are valid.
            ///
            /// This option has no effect if the [`Builder::dynamic_library_name`] option is not
            /// set.
            ///
            /// This option is disabled by default.
            pub fn dynamic_link_require_all(mut self, req: bool) -> Self {
                self.options.dynamic_link_require_all = req;
                self
            }
        },
        as_args: "--dynamic-link-require-all",
    },
    /// Only make generated bindings `pub` if the items would be publically accessible by C++.
    respect_cxx_access_specs: {
        ty: bool,
        methods: {
            /// Set the visibility of the Rust items in the generated bindings as `pub` only if the
            /// corresponding C++ item is publically accessible.
            ///
            /// This option is disabled by default.
            pub fn respect_cxx_access_specs(mut self, doit: bool) -> Self {
                self.options.respect_cxx_access_specs = doit;
                self
            }

        },
        as_args: "--respect-cxx-access-specs",
    },
    /// Always translate `enum` integer types to native Rust integer types.
    translate_enum_integer_types: {
        ty: bool,
        methods: {
            /// Always translate `enum` integer types to native Rust integer types.
            ///
            /// This will result in `enum`s having types such as `u32` and `i16` instead of
            /// `c_uint` and `c_short`. The `#[repr]` types of Rustified `enum`s are always
            /// translated to Rust integer types.
            ///
            /// This option is disabled by default.
            pub fn translate_enum_integer_types(mut self, doit: bool) -> Self {
                self.options.translate_enum_integer_types = doit;
                self
            }
        },
        as_args: "--translate-enum-integer-types",
    },
    /// Generate types with C style naming.
    c_naming: {
        ty: bool,
        methods: {
            /// Generate types with C style naming.
            ///
            /// This will add prefixes to the generated type names. For example, instead of a
            /// `struct` with name `A` we will generate a `struct` with `struct_A`. Currently
            /// applies to `struct`s, `union`s, and `enum`s.
            ///
            /// This option is disabled by default.
            pub fn c_naming(mut self, doit: bool) -> Self {
                self.options.c_naming = doit;
                self
            }
        },
        as_args: "--c-naming",
    },
    /// Always output explicit padding fields
    force_explicit_padding: {
        ty: bool,
        methods: {
            /// Set whether to always emit explicit padding fields.
            ///
            /// This option should be enabled if a `struct` needs to be serialized in its native
            /// format (padding bytes and all). This could be required if such `struct` will be
            /// written to a file or sent over the network, as anything reading the padding bytes
            /// of a struct may lead to Undefined Behavior.
            ///
            /// This option is disabled by default.
            pub fn explicit_padding(mut self, doit: bool) -> Self {
                self.options.force_explicit_padding = doit;
                self
            }
        },
        as_args: "--explicit-padding",
    },
    /// Emit vtable functions.
    vtable_generation: {
        ty: bool,
        methods: {
            /// Set whether to enable experimental support to generate vtable functions.
            ///
            /// This option should mostly work, though some edge cases are likely to be broken.
            ///
            /// This option is disabled by default.
            pub fn vtable_generation(mut self, doit: bool) -> Self {
                self.options.vtable_generation = doit;
                self
            }
        },
        as_args: "--vtable-generation",
    },
    /// Sort the generated Rust items.
    sort_semantically: {
        ty: bool,
        methods: {
            /// Set whether to sort the generated Rust items in a predefined manner.
            ///
            /// This option is disabled by default.
            pub fn sort_semantically(mut self, doit: bool) -> Self {
                self.options.sort_semantically = doit;
                self
            }
        },
        as_args: "--sort-semantically",
    },
    /// Whether to deduplicate `extern` blocks.
    merge_extern_blocks: {
        ty: bool,
        methods: {
            /// Merge all extern blocks under the same module into a single one.
            ///
            /// This option is disabled by default.
            pub fn merge_extern_blocks(mut self, doit: bool) -> Self {
                self.options.merge_extern_blocks = doit;
                self
            }
        },
        as_args: "--merge-extern-blocks",
    },
    /// Whether to wrap unsafe operations in unsafe blocks.
    wrap_unsafe_ops: {
        ty: bool,
        methods: {
            /// Wrap all unsafe operations in unsafe blocks.
            ///
            /// This option is disabled by default
            pub fn wrap_unsafe_ops(mut self, doit: bool) -> Self {
                self.options.wrap_unsafe_ops = doit;
                self
            }
        },
        as_args: "--wrap-unsafe-ops",
    },
    /// Patterns for functions whose ABI should be overriden.
    abi_overrides: {
        ty: HashMap<Abi, RegexSet>,
        methods: {
            regex_option! {
                /// Override the ABI of a given function.
                pub fn override_abi<T: Into<String>>(mut self, abi: Abi, arg: T) -> Self {
                    self.options
                        .abi_overrides
                        .entry(abi)
                        .or_default()
                        .insert(arg.into());
                    self
                }
            }
        },
        as_args: |overrides, args| {
            for (abi, set) in overrides {
                for item in set.get_items() {
                    args.push("--override-abi".to_owned());
                    args.push(format!("{}={}", item, abi));
                }
            }
        },
    },
    /// If true, generate function wrappers for `static` C functions.
    wrap_static_fns: {
        ty: bool,
        methods: {
            #[cfg(feature = "experimental")]
            /// Whether to generate extern wrappers for `static` and `static inline` functions.
            /// Defaults to `false`.
            pub fn wrap_static_fns(mut self, doit: bool) -> Self {
                self.options.wrap_static_fns = doit;
                self
            }
        },
        as_args: "--wrap-static-fns",
    },
    /// The suffix to be added to the function wrappers for `static` functions.
    wrap_static_fns_suffix: {
        ty: Option<String>,
        methods: {
            #[cfg(feature = "experimental")]
            /// Set the suffix added to the extern wrapper functions generated for `static` and `static
            /// inline` functions.
            pub fn wrap_static_fns_suffix<T: AsRef<str>>(mut self, suffix: T) -> Self {
                self.options.wrap_static_fns_suffix = Some(suffix.as_ref().to_owned());
                self
            }
        },
        as_args: "--wrap-static-fns-suffix",
    },
    /// The path of the file where the function wrappers for `static` functions will be emitted.
    wrap_static_fns_path: {
        ty: Option<PathBuf>,
        methods: {
            #[cfg(feature = "experimental")]
            /// Set the path for the source code file that would be created if any wrapper functions must
            /// be generated due to the presence of static functions.
            ///
            /// Bindgen will automatically add the right extension to the header and source code files.
            pub fn wrap_static_fns_path<T: AsRef<Path>>(mut self, path: T) -> Self {
                self.options.wrap_static_fns_path = Some(path.as_ref().to_owned());
                self
            }
        },
        as_args: "--wrap-static-fns-path",
    },
    /// Default visibility of structs and their fields.
    default_visibility: {
        ty: FieldVisibilityKind,
        methods: {
            /// Set the default visibility of fields, including bitfields and accessor methods for
            /// bitfields.
            ///
            /// This option is ignored if the [`Builder::respect_cxx_access_specs`] method is enabled.
            pub fn default_visibility(
                mut self,
                visibility: FieldVisibilityKind,
            ) -> Self {
                self.options.default_visibility = visibility;
                self
            }
        },
        as_args: |visibility, args| {
            if *visibility != Default::default() {
                args.push("--default-visibility".to_owned());
                args.push(visibility.to_string());
            }
        },
    },
}

/// Trait used to turn [`BindgenOptions`] fields into CLI args.
trait AsArgs {
    fn as_args(&self, args: &mut Vec<String>, flag: &str);
}

/// If the `bool` is `true`, `flag` is pushed into `args`.
///
/// be careful about the truth value of the field as some options, like `--no-layout-tests`, are
/// actually negations of the fields.
impl AsArgs for bool {
    fn as_args(&self, args: &mut Vec<String>, flag: &str) {
        if *self {
            args.push(flag.to_string());
        }
    }
}

/// Iterate over all the items of the `RegexSet` and push `flag` followed by the item into `args`
/// for each item.
impl AsArgs for RegexSet {
    fn as_args(&self, args: &mut Vec<String>, flag: &str) {
        for item in self.get_items() {
            args.extend_from_slice(&[flag.to_owned(), item.clone()]);
        }
    }
}

/// If the `Option` is `Some(value)`, push `flag` followed by `value`.
impl AsArgs for Option<String> {
    fn as_args(&self, args: &mut Vec<String>, flag: &str) {
        if let Some(string) = self {
            args.extend_from_slice(&[flag.to_owned(), string.clone()]);
        }
    }
}

/// If the `Option` is `Some(path)`, push `flag` followed by the [`PathBuf::display`]
/// representation of `path`.
impl AsArgs for Option<PathBuf> {
    fn as_args(&self, args: &mut Vec<String>, flag: &str) {
        if let Some(path) = self {
            args.extend_from_slice(&[
                flag.to_owned(),
                path.display().to_string(),
            ]);
        }
    }
}
