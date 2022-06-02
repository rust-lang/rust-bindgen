use crate::options::LinkNameOverrideParseCallback;
use bindgen::callbacks::*;

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
                let lnopc = LinkNameOverrideParseCallback::new(prefix.unwrap());
                Box::new(lnopc)
            } else {
                panic!("Couldn't find name ParseCallbacks: {}", cb)
            }
        }
    }
}
