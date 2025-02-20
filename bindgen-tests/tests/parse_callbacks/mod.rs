mod item_discovery_callback;

use bindgen::callbacks::*;
use bindgen::FieldVisibilityKind;

#[derive(Debug)]
pub struct RemovePrefixParseCallback {
    pub remove_prefix: Option<String>,
}

impl RemovePrefixParseCallback {
    pub fn new(prefix: &str) -> Self {
        RemovePrefixParseCallback {
            remove_prefix: Some(prefix.to_string()),
        }
    }
}

impl ParseCallbacks for RemovePrefixParseCallback {
    fn generated_name_override(&self, item_info: ItemInfo) -> Option<String> {
        if let Some(prefix) = &self.remove_prefix {
            let (expected_prefix, expected_suffix) = match item_info.kind {
                ItemKind::Function => ("function_", "_name"),
                ItemKind::Var => ("var_", "_name"),
                _ => todo!(),
            };
            if let Some(name) = item_info.name.strip_prefix(prefix) {
                assert!(name.starts_with(expected_prefix));
                assert!(name.ends_with(expected_suffix));
                return Some(name.to_string());
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct PrefixLinkNameParseCallback {
    pub prefix: Option<String>,
}

impl PrefixLinkNameParseCallback {
    pub fn new(prefix: &str) -> Self {
        PrefixLinkNameParseCallback {
            prefix: Some(prefix.to_string()),
        }
    }
}

impl ParseCallbacks for PrefixLinkNameParseCallback {
    fn generated_link_name_override(
        &self,
        item_info: ItemInfo,
    ) -> Option<String> {
        self.prefix
            .as_deref()
            .map(|prefix| format!("{prefix}{}", item_info.name))
    }
}

#[derive(Debug)]
struct EnumVariantRename;

impl ParseCallbacks for EnumVariantRename {
    fn enum_variant_name(
        &self,
        _enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: EnumVariantValue,
    ) -> Option<String> {
        Some(format!("RENAMED_{original_variant_name}"))
    }
}

#[derive(Debug)]
struct BlocklistedTypeImplementsTrait;

impl ParseCallbacks for BlocklistedTypeImplementsTrait {
    fn blocklisted_type_implements_trait(
        &self,
        _name: &str,
        derive_trait: DeriveTrait,
    ) -> Option<ImplementsTrait> {
        if derive_trait == DeriveTrait::Hash {
            Some(ImplementsTrait::No)
        } else {
            Some(ImplementsTrait::Yes)
        }
    }
}

#[derive(Debug)]
struct FieldVisibility {
    default: FieldVisibilityKind,
}

/// Implements the `field_visibility` function of the trait by checking if the
/// field name starts with `private_`. If it does, it makes it private, if it
/// doesn't, it makes it public, taking into account the default visibility.
impl ParseCallbacks for FieldVisibility {
    fn field_visibility(
        &self,
        FieldInfo { field_name, .. }: FieldInfo,
    ) -> Option<FieldVisibilityKind> {
        match (self.default, field_name.starts_with("private_")) {
            (FieldVisibilityKind::Private, false) => {
                Some(FieldVisibilityKind::Public)
            }
            (FieldVisibilityKind::Public, true) => {
                Some(FieldVisibilityKind::Private)
            }
            (FieldVisibilityKind::PublicCrate, _) => unimplemented!(),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct TypeVisibility;

/// Implements the `field_visibility` function of the trait by checking the
/// type name. Depending on name prefix, it will return a different visibility.
impl ParseCallbacks for TypeVisibility {
    fn field_visibility(
        &self,
        FieldInfo { type_name, .. }: FieldInfo,
    ) -> Option<FieldVisibilityKind> {
        if type_name.starts_with("private_") {
            Some(FieldVisibilityKind::Private)
        } else if type_name.starts_with("pubcrate_") {
            Some(FieldVisibilityKind::PublicCrate)
        } else if type_name.starts_with("pub_") {
            Some(FieldVisibilityKind::Public)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub(super) struct WrapAsVariadicFn;

impl ParseCallbacks for WrapAsVariadicFn {
    fn wrap_as_variadic_fn(&self, name: &str) -> Option<String> {
        Some(name.to_owned() + "_wrapped")
    }
}

#[derive(Debug)]
struct DeriveTransparentSerialize(String);

impl ParseCallbacks for DeriveTransparentSerialize {
    fn add_derives(&self, info: &DeriveInfo<'_>) -> Vec<String> {
        if info.name == &self.0 {
            vec!["serde::Serialize".to_owned()]
        } else {
            vec![]
        }
    }

    fn add_attributes(&self, info: &AttributeInfo<'_>) -> Vec<String> {
        if info.name == &self.0 {
            vec!["#[serde(rename_all = \"UPPERCASE\")]".to_owned()]
        } else {
            vec![]
        }
    }
}

pub fn lookup(cb: &str) -> Box<dyn ParseCallbacks> {
    match cb {
        "enum-variant-rename" => Box::new(EnumVariantRename),
        "blocklisted-type-implements-trait" => {
            Box::new(BlocklistedTypeImplementsTrait)
        }
        "wrap-as-variadic-fn" => Box::new(WrapAsVariadicFn),
        "type-visibility" => Box::new(TypeVisibility),
        call_back => {
            if let Some(name) =
                call_back.strip_prefix("derive-uppercase-serialize=")
            {
                Box::new(DeriveTransparentSerialize(name.to_owned()))
            } else if let Some(prefix) =
                call_back.strip_prefix("remove-function-prefix-")
            {
                let lnopc = RemovePrefixParseCallback::new(prefix);
                Box::new(lnopc)
            } else if let Some(prefix) =
                call_back.strip_prefix("prefix-link-name-")
            {
                let plnpc = PrefixLinkNameParseCallback::new(prefix);
                Box::new(plnpc)
            } else if let Some(default) =
                call_back.strip_prefix("field-visibility-default-")
            {
                Box::new(FieldVisibility {
                    default: default.parse().expect(
                        "unable to parse field-visibility-default callback",
                    ),
                })
            } else {
                panic!("Couldn't find name ParseCallbacks: {cb}")
            }
        }
    }
}
