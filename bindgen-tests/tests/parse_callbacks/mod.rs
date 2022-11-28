use bindgen::callbacks::*;

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
struct EnumVariantRename;

impl ParseCallbacks for EnumVariantRename {
    fn enum_variant_name(
        &self,
        _enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: EnumVariantValue,
    ) -> Option<String> {
        Some(format!("RENAMED_{}", original_variant_name))
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

pub fn lookup(cb: &str) -> Box<dyn ParseCallbacks> {
    match cb {
        "enum-variant-rename" => Box::new(EnumVariantRename),
        "blocklisted-type-implements-trait" => {
            Box::new(BlocklistedTypeImplementsTrait)
        }
        call_back => {
            if call_back.starts_with("remove-function-prefix-") {
                let prefix = call_back
                    .split("remove-function-prefix-")
                    .last()
                    .to_owned();
                let lnopc = RemovePrefixParseCallback::new(prefix.unwrap());
                Box::new(lnopc)
            } else {
                panic!("Couldn't find name ParseCallbacks: {}", cb)
            }
        }
    }
}
