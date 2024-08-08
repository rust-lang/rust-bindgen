//! A public API for more fine-grained customization of bindgen behavior.

pub use crate::ir::analysis::DeriveTrait;
pub use crate::ir::derive::CanDerive as ImplementsTrait;
pub use crate::ir::enum_ty::{EnumVariantCustomBehavior, EnumVariantValue};
pub use crate::ir::int::IntKind;
use std::fmt;

/// An enum to allow ignoring parsing of macros.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum MacroParsingBehavior {
    /// Ignore the macro, generating no code for it, or anything that depends on
    /// it.
    Ignore,
    /// The default behavior bindgen would have otherwise.
    #[default]
    Default,
}

/// A trait to allow configuring different kinds of types in different
/// situations.
pub trait ParseCallbacks: fmt::Debug {
    #[cfg(feature = "__cli")]
    #[doc(hidden)]
    fn cli_args(&self) -> Vec<String> {
        vec![]
    }

    /// This function will be run on every macro that is identified.
    #[allow(unused_variables)]
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        MacroParsingBehavior::Default
    }

    /// This function will run for every extern variable and function. The returned value determines
    /// the name visible in the bindings.
    #[allow(unused_variables)]
    fn generated_name_override(
        &self,
        item_info: ItemInfo<'_>,
    ) -> Option<String> {
        None
    }

    /// This function will run for every extern variable and function. The returned value determines
    /// the link name in the bindings.
    #[allow(unused_variables)]
    fn generated_link_name_override(
        &self,
        item_info: ItemInfo<'_>,
    ) -> Option<String> {
        None
    }

    /// The integer kind an integer macro should have, given a name and the
    /// value of that macro, or `None` if you want the default to be chosen.
    #[allow(unused_variables)]
    fn int_macro(&self, name: &str, value: i128) -> Option<IntKind> {
        None
    }

    /// This will be run on every string macro. The callback cannot influence the further
    /// treatment of the macro, but may use the value to generate additional code or configuration.
    #[allow(unused_variables)]
    fn str_macro(&self, name: &str, value: &[u8]) {}

    /// This will be run on every function-like macro. The callback cannot
    /// influence the further treatment of the macro, but may use the value to
    /// generate additional code or configuration.
    #[allow(unused_variables)]
    fn fn_macro(&self, info: &FnMacroInfo<'_>) {}

    /// This function should return whether, given an enum variant
    /// name, and value, this enum variant will forcibly be a constant.
    #[allow(unused_variables)]
    fn enum_variant_behavior(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        variant_value: EnumVariantValue,
    ) -> Option<EnumVariantCustomBehavior> {
        None
    }

    /// Allows to rename an enum variant, replacing `_original_variant_name`.
    #[allow(unused_variables)]
    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        variant_value: EnumVariantValue,
    ) -> Option<String> {
        None
    }

    /// Allows to rename an item, replacing `_original_item_name`.
    #[allow(unused_variables)]
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        None
    }

    /// This will be called on every header filename passed to (`Builder::header`)[`crate::Builder::header`].
    #[allow(unused_variables)]
    fn header_file(&self, filename: &str) {}

    /// This will be called on every file inclusion, with the full path of the included file.
    #[allow(unused_variables)]
    fn include_file(&self, filename: &str) {}

    /// This will be called every time `bindgen` reads an environment variable whether it has any
    /// content or not.
    #[allow(unused_variables)]
    fn read_env_var(&self, key: &str) {}

    /// This will be called to determine whether a particular blocklisted type
    /// implements a trait or not. This will be used to implement traits on
    /// other types containing the blocklisted type.
    ///
    /// * `None`: use the default behavior
    /// * `Some(ImplementsTrait::Yes)`: `_name` implements `_derive_trait`
    /// * `Some(ImplementsTrait::Manually)`: any type including `_name` can't
    ///   derive `_derive_trait` but can implemented it manually
    /// * `Some(ImplementsTrait::No)`: `_name` doesn't implement `_derive_trait`
    #[allow(unused_variables)]
    fn blocklisted_type_implements_trait(
        &self,
        name: &str,
        derive_trait: DeriveTrait,
    ) -> Option<ImplementsTrait> {
        None
    }

    /// Provide a list of custom derive attributes.
    ///
    /// If no additional attributes are wanted, this function should return an
    /// empty `Vec`.
    #[allow(unused_variables)]
    fn add_derives(&self, info: &DeriveInfo<'_>) -> Vec<String> {
        vec![]
    }

    /// Process a source code comment.
    #[allow(unused_variables)]
    fn process_comment(&self, comment: &str) -> Option<String> {
        None
    }

    /// Potentially override the visibility of a composite type field.
    ///
    /// Caution: This allows overriding standard C++ visibility inferred by
    /// `respect_cxx_access_specs`.
    #[allow(unused_variables)]
    fn field_visibility(
        &self,
        info: FieldInfo<'_>,
    ) -> Option<crate::FieldVisibilityKind> {
        None
    }

    /// Process a function name that as exactly one `va_list` argument
    /// to be wrapped as a variadic function with the wrapped static function
    /// feature.
    ///
    /// The returned string is new function name.
    #[cfg(feature = "experimental")]
    #[allow(unused_variables)]
    fn wrap_as_variadic_fn(&self, name: &str) -> Option<String> {
        None
    }
}

/// Relevant information about a type to which new derive attributes will be added using
/// [`ParseCallbacks::add_derives`].
#[derive(Debug)]
#[non_exhaustive]
pub struct DeriveInfo<'a> {
    /// The name of the type.
    pub name: &'a str,
    /// The kind of the type.
    pub kind: TypeKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The kind of the current type.
pub enum TypeKind {
    /// The type is a Rust `struct`.
    Struct,
    /// The type is a Rust `enum`.
    Enum,
    /// The type is a Rust `union`.
    Union,
}

/// A struct providing information about the item being passed to [`ParseCallbacks::generated_name_override`].
#[non_exhaustive]
pub struct ItemInfo<'a> {
    /// The name of the item
    pub name: &'a str,
    /// The kind of item
    pub kind: ItemKind,
}

/// An enum indicating the kind of item for an ItemInfo.
#[non_exhaustive]
pub enum ItemKind {
    /// A Function
    Function,
    /// A Variable
    Var,
}

/// Relevant information about a field for which visibility can be determined using
/// [`ParseCallbacks::field_visibility`].
#[derive(Debug)]
#[non_exhaustive]
pub struct FieldInfo<'a> {
    /// The name of the type.
    pub type_name: &'a str,
    /// The name of the field.
    pub field_name: &'a str,
}

/// A struct providing information about the function-like macro being passed to [`ParseCallbacks::fn_macro`].
pub struct FnMacroInfo<'m> {
    pub(crate) name: &'m str,
    pub(crate) args: &'m [&'m str],
    pub(crate) body: &'m [&'m str],
}

impl FnMacroInfo<'_> {
    /// The macro name.
    pub fn name(&self) -> &str {
        self.name
    }

    /// The macro argument names.
    pub fn args(&self) -> &[&str] {
        self.args
    }

    /// The macro body as delimited `clang` tokens.
    pub fn body(&self) -> &[&str] {
        self.body
    }
}
