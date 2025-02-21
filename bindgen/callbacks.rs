//! A public API for more fine-grained customization of bindgen behavior.

pub use crate::ir::analysis::DeriveTrait;
pub use crate::ir::comp::SpecialMemberKind;
pub use crate::ir::derive::CanDerive as ImplementsTrait;
pub use crate::ir::enum_ty::{EnumVariantCustomBehavior, EnumVariantValue};
pub use crate::ir::function::Explicitness;
pub use crate::ir::function::Visibility;
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
    fn will_parse_macro(&self, _name: &str) -> MacroParsingBehavior {
        MacroParsingBehavior::Default
    }

    /// This function will run for every extern variable and function. The returned value determines
    /// the name visible in the bindings.
    fn generated_name_override(
        &self,
        _item_info: ItemInfo<'_>,
    ) -> Option<String> {
        None
    }

    /// This function will run for every extern variable and function. The returned value determines
    /// the link name in the bindings.
    fn generated_link_name_override(
        &self,
        _item_info: ItemInfo<'_>,
    ) -> Option<String> {
        None
    }

    /// The integer kind an integer macro should have, given a name and the
    /// value of that macro, or `None` if you want the default to be chosen.
    fn int_macro(&self, _name: &str, _value: i64) -> Option<IntKind> {
        None
    }

    /// This will be run on every string macro. The callback cannot influence the further
    /// treatment of the macro, but may use the value to generate additional code or configuration.
    fn str_macro(&self, _name: &str, _value: &[u8]) {}

    /// This will be run on every function-like macro. The callback cannot
    /// influence the further treatment of the macro, but may use the value to
    /// generate additional code or configuration.
    ///
    /// The first parameter represents the name and argument list (including the
    /// parentheses) of the function-like macro. The second parameter represents
    /// the expansion of the macro as a sequence of tokens.
    fn func_macro(&self, _name: &str, _value: &[&[u8]]) {}

    /// This function should return whether, given an enum variant
    /// name, and value, this enum variant will forcibly be a constant.
    fn enum_variant_behavior(
        &self,
        _enum_name: Option<&str>,
        _original_variant_name: &str,
        _variant_value: EnumVariantValue,
    ) -> Option<EnumVariantCustomBehavior> {
        None
    }

    /// Allows to rename an enum variant, replacing `_original_variant_name`.
    fn enum_variant_name(
        &self,
        _enum_name: Option<&str>,
        _original_variant_name: &str,
        _variant_value: EnumVariantValue,
    ) -> Option<String> {
        None
    }

    /// Allows to rename an item, replacing `_original_item_name`.
    fn item_name(&self, _original_item_name: &str) -> Option<String> {
        None
    }

    /// This will be called on every header filename passed to (`Builder::header`)[`crate::Builder::header`].
    fn header_file(&self, _filename: &str) {}

    /// This will be called on every file inclusion, with the full path of the included file.
    fn include_file(&self, _filename: &str) {}

    /// This will be called every time `bindgen` reads an environment variable whether it has any
    /// content or not.
    fn read_env_var(&self, _key: &str) {}

    /// This will be called to determine whether a particular blocklisted type
    /// implements a trait or not. This will be used to implement traits on
    /// other types containing the blocklisted type.
    ///
    /// * `None`: use the default behavior
    /// * `Some(ImplementsTrait::Yes)`: `_name` implements `_derive_trait`
    /// * `Some(ImplementsTrait::Manually)`: any type including `_name` can't
    ///   derive `_derive_trait` but can implemented it manually
    /// * `Some(ImplementsTrait::No)`: `_name` doesn't implement `_derive_trait`
    fn blocklisted_type_implements_trait(
        &self,
        _name: &str,
        _derive_trait: DeriveTrait,
    ) -> Option<ImplementsTrait> {
        None
    }

    /// Provide a list of custom derive attributes.
    ///
    /// If no additional attributes are wanted, this function should return an
    /// empty `Vec`.
    fn add_derives(&self, _info: &DeriveInfo<'_>) -> Vec<String> {
        vec![]
    }

    /// Provide a list of custom attributes.
    ///
    /// If no additional attributes are wanted, this function should return an
    /// empty `Vec`.
    fn add_attributes(&self, _info: &AttributeInfo<'_>) -> Vec<String> {
        vec![]
    }

    /// Process a source code comment.
    fn process_comment(&self, _comment: &str) -> Option<String> {
        None
    }

    /// Potentially override the visibility of a composite type field.
    ///
    /// Caution: This allows overriding standard C++ visibility inferred by
    /// `respect_cxx_access_specs`.
    fn field_visibility(
        &self,
        _info: FieldInfo<'_>,
    ) -> Option<crate::FieldVisibilityKind> {
        None
    }

    /// Process a function name that as exactly one `va_list` argument
    /// to be wrapped as a variadic function with the wrapped static function
    /// feature.
    ///
    /// The returned string is new function name.
    #[cfg(feature = "experimental")]
    fn wrap_as_variadic_fn(&self, _name: &str) -> Option<String> {
        None
    }

    /// This will get called everytime an item is found with some information about it.
    /// `_parent` is the location in which the item has been found, if any.
    /// This is guaranteed to be a [`DiscoveredItem`] as reported
    /// by [`ParseCallbacks::new_item_found`], most likely a
    /// [`DiscoveredItem::Mod`] but perhaps something else such as a
    /// [`DiscoveredItem::Struct`].
    /// If C++ namespace support has not been enabled in bindgen's options,
    /// most items will have no declared `_parent`. If C++ namespace support
    /// has been enabled, all items should have a parent other than the root
    /// namespace.
    fn new_item_found(
        &self,
        _id: DiscoveredItemId,
        _item: DiscoveredItem,
        _source_location: Option<&SourceLocation>,
        _parent: Option<DiscoveredItemId>,
    ) {
    }

    /// Notes that this type does not use all the template params
    /// which were present in C++. Sometimes those template parameters
    /// are not useful in Rust because they don't actually impact the
    /// data stored in the type, so bindgen drops them. But this also means
    /// that the bindgen output does not contain full and comprehensive
    /// output about the original nature of the C++ type, so higher level
    /// code generators may wish to behave differently. For example,
    /// it would not be OK to attempt to generate additional C++ code
    /// based on this.
    fn denote_discards_template_param(&self, _id: DiscoveredItemId) {}

    // TODO add callback for ResolvedTypeRef
}

/// An identifier for a discovered item. Used to identify an aliased type (see [`DiscoveredItem::Alias`])
#[derive(Ord, PartialOrd, PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct DiscoveredItemId(usize);

impl DiscoveredItemId {
    /// Constructor
    pub fn new(value: usize) -> Self {
        Self(value)
    }
}

/// Struct passed to [`ParseCallbacks::new_item_found`] containing information about discovered
/// items (struct, union, and alias)
#[derive(Debug, Hash, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum DiscoveredItem {
    /// Represents a struct with its original name in C and its generated binding name
    Struct {
        /// The original name (learnt from C) of the structure
        /// Can be None if the union is anonymous.
        original_name: Option<String>,

        /// The name of the generated binding
        final_name: String,

        /// Its C++ visibility. [`Visibility::Public`] unless this is nested
        /// in another type.
        cpp_visibility: Visibility,
    },

    /// Represents a union with its original name in C and its generated binding name
    Union {
        /// The original name (learnt from C) of the structure.
        /// Can be None if the union is anonymous.
        original_name: Option<String>,

        /// The name of the generated binding
        final_name: String,

        /// Its C++ visibility. [`Visibility::Public`] unless this is nested
        /// in another type.
        cpp_visibility: Visibility,
    },

    /// Represents an alias like a typedef
    /// ```c
    ///     typedef struct MyStruct {
    ///         ...
    ///     } StructAlias;
    /// ```
    /// Here, the name of the alias is `StructAlias` and it's an alias for `MyStruct`
    Alias {
        /// The name of the alias in C (`StructAlias`)
        alias_name: String,

        /// The identifier of the discovered type
        alias_for: DiscoveredItemId,
    },

    /// Represents an enum.
    Enum {
        /// The final name of the generated binding
        final_name: String,

        /// Its C++ visibility. [`Visibility::Public`] unless this is nested
        /// in another type.
        cpp_visibility: Visibility,
    },

    /// A module, representing a C++ namespace.
    /// The root module can be identified by the fact that it has a `None`
    /// parent declared within [`ParseCallbacks::new_item_found`].
    /// Inline namespaces won't be reported at all unless the
    /// "enable conservative inline namespaces" option is enabled.
    Mod {
        /// The final name used.
        final_name: String,
        /// Whether this was originally an anonymous namespace.
        /// bindgen will have assigned a name within `final_name`.
        anonymous: bool,
        /// Whether this is an inline namespace.
        inline: bool,
    },

    /// A function or method.
    Function {
        /// The final name used.
        final_name: String,
    },

    /// A method.
    Method {
        /// The final name used.
        final_name: String,

        /// Type to which this method belongs.
        parent: DiscoveredItemId,

        /// Its C++ visibility.
        cpp_visibility: Visibility,

        /// Whether this is a C++ "special member".
        cpp_special_member: Option<SpecialMemberKind>,

        /// Whether this is a C++ virtual function.
        cpp_virtual: Option<Virtualness>,

        /// Whether this is a C++ function which has been marked
        /// `=default` or `=deleted`. Note that deleted functions aren't
        /// normally generated without special bindgen options.
        cpp_explicit: Option<Explicitness>,
    },
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

/// Relevant information about a type to which new attributes will be added using
/// [`ParseCallbacks::add_attributes`].
#[derive(Debug)]
#[non_exhaustive]
pub struct AttributeInfo<'a> {
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

/// An enum indicating the kind of item for an `ItemInfo`.
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
    /// The name of the type of the field.
    pub field_type_name: Option<&'a str>,
}

/// Whether a method is virtual or pure virtual.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Virtualness {
    /// Not pure virtual.
    Virtual,
    /// Pure virtual.
    PureVirtual,
}

/// Location in the source code. Roughly equivalent to the same type
/// within `clang_sys`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceLocation {
    /// Line number.
    pub line: usize,
    /// Column number within line.
    pub col: usize,
    /// Byte offset within file.
    pub byte_offset: usize,
    /// Filename, if known.
    pub file_name: Option<String>,
}
