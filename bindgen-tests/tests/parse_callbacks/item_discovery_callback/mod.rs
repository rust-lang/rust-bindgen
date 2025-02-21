use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::identity;
use std::rc::Rc;

use regex::Regex;

use bindgen::callbacks::{
    DiscoveredItem, DiscoveredItemId, ParseCallbacks, SourceLocation,
};
use bindgen::Builder;

#[derive(Debug, Default)]
struct ItemDiscovery(Rc<RefCell<ItemCache>>);

pub type ItemCache = HashMap<DiscoveredItemId, DiscoveredInformation>;

#[derive(Debug)]
pub struct DiscoveredInformation(
    DiscoveredItem,
    Option<SourceLocation>,
    Option<DiscoveredItemId>,
);

impl ParseCallbacks for ItemDiscovery {
    fn new_item_found(
        &self,
        id: DiscoveredItemId,
        item: DiscoveredItem,
        source_location: Option<&SourceLocation>,
        parent: Option<DiscoveredItemId>,
    ) {
        self.0.borrow_mut().insert(
            id,
            DiscoveredInformation(item, source_location.cloned(), parent),
        );
    }
}

#[derive(Debug)]
pub struct ItemExpectations {
    item: DiscoveredItem,
    source_location: Option<(usize, usize, usize)>,
    parent: Option<DiscoveredItemId>,
}

impl ItemExpectations {
    fn new(
        item: DiscoveredItem,
        line: usize,
        col: usize,
        byte_offset: usize,
        parent: Option<DiscoveredItemId>,
    ) -> Self {
        Self {
            item,
            source_location: Some((line, col, byte_offset)),
            parent,
        }
    }

    fn new_no_source_location(
        item: DiscoveredItem,
        parent: Option<DiscoveredItemId>,
    ) -> Self {
        Self {
            item,
            source_location: None,
            parent,
        }
    }
}

type ExpectationMap = HashMap<DiscoveredItemId, ItemExpectations>;

fn test_item_discovery_callback<F: FnOnce(Builder) -> Builder>(
    header: &str,
    expected: HashMap<DiscoveredItemId, ItemExpectations>,
    builder_adjuster: F,
) {
    let discovery = ItemDiscovery::default();
    let info = Rc::clone(&discovery.0);

    let mut header_path = env!("CARGO_MANIFEST_DIR").to_string();
    header_path.push_str(header);

    let b = Builder::default()
        .header(header_path)
        .parse_callbacks(Box::new(discovery));
    builder_adjuster(b).generate().expect("TODO: panic message");

    compare_item_caches(&info.borrow(), &expected, header);
}

#[test]
fn test_item_discovery_callback_c() {
    let expected = ExpectationMap::from([
        (
            DiscoveredItemId::new(10),
            ItemExpectations::new(
                DiscoveredItem::Struct {
                    original_name: Some("NamedStruct".to_string()),
                    final_name: "NamedStruct".to_string(),
                },
                4,
                8,
                73,
                None,
            ),
        ),
        (
            DiscoveredItemId::new(11),
            ItemExpectations::new(
                DiscoveredItem::Alias {
                    alias_name: "AliasOfNamedStruct".to_string(),
                    alias_for: DiscoveredItemId::new(10),
                },
                7,
                28,
                118,
                None,
            ),
        ),
        (
            DiscoveredItemId::new(20),
            ItemExpectations::new(
                DiscoveredItem::Union {
                    original_name: Some("NamedUnion".to_string()),
                    final_name: "NamedUnion".to_string(),
                },
                13,
                7,
                209,
                None,
            ),
        ),
        (
            DiscoveredItemId::new(21),
            ItemExpectations::new(
                DiscoveredItem::Alias {
                    alias_name: "AliasOfNamedUnion".to_string(),
                    alias_for: DiscoveredItemId::new(20),
                },
                16,
                26,
                251,
                None,
            ),
        ),
        (
            DiscoveredItemId::new(27),
            ItemExpectations::new(
                DiscoveredItem::Alias {
                    alias_name: "AliasOfNamedEnum".to_string(),
                    alias_for: DiscoveredItemId::new(24),
                },
                28,
                24,
                515,
                None,
            ),
        ),
        (
            DiscoveredItemId::new(24),
            ItemExpectations::new(
                DiscoveredItem::Enum {
                    final_name: "NamedEnum".to_string(),
                },
                24,
                6,
                466,
                None,
            ),
        ),
        (
            DiscoveredItemId::new(30),
            ItemExpectations::new(
                DiscoveredItem::Struct {
                    original_name: None,
                    final_name: "_bindgen_ty_*".to_string(),
                },
                2,
                38,
                48,
                None,
            ),
        ),
        (
            DiscoveredItemId::new(40),
            ItemExpectations::new(
                DiscoveredItem::Union {
                    original_name: None,
                    final_name: "_bindgen_ty_*".to_string(),
                },
                11,
                37,
                186,
                None,
            ),
        ),
        (
            DiscoveredItemId::new(41),
            ItemExpectations::new(
                DiscoveredItem::Function {
                    final_name: "named_function".to_string(),
                },
                32,
                6,
                553,
                None,
            ),
        ),
    ]);
    test_item_discovery_callback(
        "/tests/parse_callbacks/item_discovery_callback/header_item_discovery.h", expected, identity);
}

#[test]
fn test_item_discovery_callback_cpp() {
    let expected = ExpectationMap::from([
        (
            DiscoveredItemId::new(1),
            ItemExpectations::new(
                DiscoveredItem::Struct {
                    original_name: Some("SomeClass".to_string()),
                    final_name: "SomeClass".to_string(),
                },
                3,
                7,
                18,
                None,
            ),
        ),
        (
            DiscoveredItemId::new(2),
            ItemExpectations::new(
                DiscoveredItem::Method {
                    final_name: "named_method".to_string(),
                    parent: DiscoveredItemId::new(1),
                },
                5,
                10,
                47,
                None,
            ),
        ),
    ]);
    test_item_discovery_callback(
        "/tests/parse_callbacks/item_discovery_callback/header_item_discovery.hpp", expected, identity);
}

/// Returns the expectations corresponding to header_item_discovery_with_namespaces.hpp,
/// other than those items whose behavior changes based on the setting for
/// conservative inline namespaces, which we test each way.
fn cpp_expectation_map() -> ExpectationMap {
    ExpectationMap::from([
        (
            DiscoveredItemId::new(0),
            ItemExpectations::new_no_source_location(
                DiscoveredItem::Mod {
                    final_name: "".to_string(),
                    anonymous: false,
                    inline: false,
                },
                None,
            ),
        ),
        (
            DiscoveredItemId::new(4),
            ItemExpectations::new(
                DiscoveredItem::Function {
                    final_name: "a".to_string(),
                },
                1,
                6,
                5,
                Some(DiscoveredItemId::new(0)),
            ),
        ),
        (
            DiscoveredItemId::new(5),
            ItemExpectations::new(
                DiscoveredItem::Mod {
                    final_name: "B".to_string(),
                    anonymous: false,
                    inline: false,
                },
                3,
                11,
                21,
                Some(DiscoveredItemId::new(0)),
            ),
        ),
        (
            DiscoveredItemId::new(9),
            ItemExpectations::new(
                DiscoveredItem::Function {
                    final_name: "c".to_string(),
                },
                4,
                10,
                34,
                Some(DiscoveredItemId::new(5)),
            ),
        ),
        (
            DiscoveredItemId::new(10),
            ItemExpectations::new(
                DiscoveredItem::Mod {
                    final_name: "D".to_string(),
                    anonymous: false,
                    inline: false,
                },
                6,
                15,
                54,
                Some(DiscoveredItemId::new(5)),
            ),
        ),
        (
            DiscoveredItemId::new(14),
            ItemExpectations::new(
                DiscoveredItem::Function {
                    final_name: "e".to_string(),
                },
                7,
                14,
                71,
                Some(DiscoveredItemId::new(10)),
            ),
        ),
        (
            DiscoveredItemId::new(16),
            ItemExpectations::new(
                DiscoveredItem::Mod {
                    final_name: "".to_string(),
                    anonymous: true,
                    inline: false,
                },
                14,
                15,
                167,
                Some(DiscoveredItemId::new(5)),
            ),
        ),
        (
            DiscoveredItemId::new(30),
            ItemExpectations::new(
                DiscoveredItem::Function {
                    final_name: "k".to_string(),
                },
                21,
                18,
                276,
                Some(DiscoveredItemId::new(26)),
            ),
        ),
        (
            DiscoveredItemId::new(31),
            ItemExpectations::new(
                DiscoveredItem::Struct {
                    final_name: "L".to_string(),
                    original_name: Some("L".to_string()),
                },
                25,
                12,
                309,
                Some(DiscoveredItemId::new(5)),
            ),
        ),
        (
            DiscoveredItemId::new(32),
            ItemExpectations::new(
                DiscoveredItem::Struct {
                    final_name: "L_M".to_string(),
                    original_name: Some("M".to_string()),
                },
                26,
                16,
                328,
                Some(DiscoveredItemId::new(31)),
            ),
        ),
    ])
}

#[test]
fn test_item_discovery_callback_cpp_namespaces_no_inline_namespaces() {
    let mut expected = cpp_expectation_map();
    expected.insert(
        DiscoveredItemId::new(25),
        ItemExpectations::new(
            DiscoveredItem::Function {
                final_name: "i".to_string(),
            },
            19,
            14,
            232,
            Some(DiscoveredItemId::new(5)),
        ),
    );
    expected.insert(
        DiscoveredItemId::new(26),
        ItemExpectations::new(
            DiscoveredItem::Mod {
                final_name: "J".to_string(),
                anonymous: false,
                inline: false,
            },
            20,
            19,
            255,
            Some(DiscoveredItemId::new(5)),
        ),
    );

    // C++11 for inline namespace
    test_item_discovery_callback(
        "/tests/parse_callbacks/item_discovery_callback/header_item_discovery_with_namespaces.hpp", expected, |b| b.enable_cxx_namespaces().clang_arg("--std=c++11"));
}

#[test]
fn test_item_discovery_callback_cpp_namespaces_with_inline_namespaces() {
    let mut expected = cpp_expectation_map();
    expected.insert(
        DiscoveredItemId::new(21),
        ItemExpectations::new(
            DiscoveredItem::Mod {
                final_name: "H".to_string(),
                anonymous: false,
                inline: true,
            },
            18,
            22,
            215,
            Some(DiscoveredItemId::new(5)),
        ),
    );
    expected.insert(
        DiscoveredItemId::new(25),
        ItemExpectations::new(
            DiscoveredItem::Function {
                final_name: "i".to_string(),
            },
            19,
            14,
            232,
            Some(DiscoveredItemId::new(21)),
        ),
    );
    expected.insert(
        DiscoveredItemId::new(26),
        ItemExpectations::new(
            DiscoveredItem::Mod {
                final_name: "J".to_string(),
                anonymous: false,
                inline: false,
            },
            20,
            19,
            255,
            Some(DiscoveredItemId::new(21)),
        ),
    );

    // C++11 for inline namespace
    test_item_discovery_callback(
        "/tests/parse_callbacks/item_discovery_callback/header_item_discovery_with_namespaces.hpp", expected, |b| b.enable_cxx_namespaces().conservative_inline_namespaces().clang_arg("--std=c++11"));
}

fn compare_item_caches(
    generated: &ItemCache,
    expected: &ExpectationMap,
    expected_filename: &str,
) {
    // We can't use a simple Eq::eq comparison because of two reasons:
    // - anonymous structs/unions will have a final name generated by bindgen which may change
    //   if the header file or the bindgen logic is altered
    // - aliases have a DiscoveredItemId that we can't directly compare for the same instability reasons
    for expected_item in expected.values() {
        let found = generated.iter().find(|(_generated_id, generated_item)| {
            compare_item_info(
                expected_item,
                generated_item,
                expected,
                generated,
                expected_filename,
            )
        });

        assert!(
            found.is_some(),
            "Missing Expected Item: {expected_item:#?}\n in {generated:#?}"
        );
    }
}

fn compare_item_info(
    expected_item: &ItemExpectations,
    generated_item: &DiscoveredInformation,
    expected: &ExpectationMap,
    generated: &ItemCache,
    expected_filename: &str,
) -> bool {
    if std::mem::discriminant(&expected_item.item)
        != std::mem::discriminant(&generated_item.0)
    {
        return false;
    }

    let is_a_match = match generated_item.0 {
        DiscoveredItem::Struct { .. } => {
            compare_struct_info(&expected_item.item, &generated_item.0)
        }
        DiscoveredItem::Union { .. } => {
            compare_union_info(&expected_item.item, &generated_item.0)
        }
        DiscoveredItem::Alias { .. } => compare_alias_info(
            &expected_item.item,
            &generated_item.0,
            expected,
            generated,
            expected_filename,
        ),
        DiscoveredItem::Enum { .. } => {
            compare_enum_info(&expected_item.item, &generated_item.0)
        }
        DiscoveredItem::Function { .. } => {
            compare_function_info(&expected_item.item, &generated_item.0)
        }
        DiscoveredItem::Method { .. } => {
            compare_method_info(&expected_item.item, &generated_item.0)
        }
        DiscoveredItem::Mod { .. } => {
            compare_mod_info(&expected_item.item, &generated_item.0)
        }
    };

    if is_a_match {
        // Compare source location
        assert!(
            generated_item.1.is_some() ==
                expected_item.source_location.is_some(),
            "Source location wasn't as expected for generated={generated_item:?}, expected={expected_item:?}"
        );
        if let Some(generated_location) = generated_item.1.as_ref() {
            let expected_location = expected_item.source_location.unwrap();
            assert!(
                generated_location
                    .file_name
                    .as_ref()
                    .expect("No filename provided")
                    .ends_with(expected_filename),
                "Filename differed"
            );
            assert_eq!(
                (
                    generated_location.line,
                    generated_location.col,
                    generated_location.byte_offset
                ),
                expected_location,
                "Line/col/offsets differ"
            );
        }

        // Compare C++ name info
        assert!(
            generated_item.2.is_some() ==
                expected_item.parent.is_some(),
            "Parent information didn't match: generated item {generated_item:?}"
        );

        if let Some(generated_parent) = generated_item.2.as_ref() {
            let expected_parent = expected_item.parent.as_ref().unwrap();
            assert_eq!(
                generated_parent, expected_parent,
                "Parent didn't match for {expected_item:?}"
            );
        }
    }
    is_a_match
}

pub fn compare_names(expected_name: &str, generated_name: &str) -> bool {
    if let Ok(regex) = Regex::new(expected_name) {
        regex.is_match(generated_name)
    } else {
        false
    }
}

pub fn compare_struct_info(
    expected_item: &DiscoveredItem,
    generated_item: &DiscoveredItem,
) -> bool {
    let DiscoveredItem::Struct {
        original_name: expected_original_name,
        final_name: expected_final_name,
    } = expected_item
    else {
        unreachable!()
    };

    let DiscoveredItem::Struct {
        original_name: generated_original_name,
        final_name: generated_final_name,
    } = generated_item
    else {
        unreachable!()
    };

    if !compare_names(expected_final_name, generated_final_name) {
        return false;
    }

    match (expected_original_name, generated_original_name) {
        (None, None) => true,
        (Some(expected_original_name), Some(generated_original_name)) => {
            compare_names(expected_original_name, generated_original_name)
        }
        _ => false,
    }
}

pub fn compare_union_info(
    expected_item: &DiscoveredItem,
    generated_item: &DiscoveredItem,
) -> bool {
    let DiscoveredItem::Union {
        original_name: expected_original_name,
        final_name: expected_final_name,
    } = expected_item
    else {
        unreachable!()
    };

    let DiscoveredItem::Union {
        original_name: generated_original_name,
        final_name: generated_final_name,
    } = generated_item
    else {
        unreachable!()
    };

    if !compare_names(expected_final_name, generated_final_name) {
        return false;
    }

    match (expected_original_name, generated_original_name) {
        (None, None) => true,
        (Some(expected_original_name), Some(generated_original_name)) => {
            compare_names(expected_original_name, generated_original_name)
        }
        _ => false,
    }
}

pub fn compare_enum_info(
    expected_item: &DiscoveredItem,
    generated_item: &DiscoveredItem,
) -> bool {
    let DiscoveredItem::Enum {
        final_name: expected_final_name,
    } = expected_item
    else {
        unreachable!()
    };

    let DiscoveredItem::Enum {
        final_name: generated_final_name,
    } = generated_item
    else {
        unreachable!()
    };

    if !compare_names(expected_final_name, generated_final_name) {
        return false;
    }
    true
}

pub fn compare_alias_info(
    expected_item: &DiscoveredItem,
    generated_item: &DiscoveredItem,
    expected: &ExpectationMap,
    generated: &ItemCache,
    expected_filename: &str,
) -> bool {
    let DiscoveredItem::Alias {
        alias_name: expected_alias_name,
        alias_for: expected_alias_for,
    } = expected_item
    else {
        unreachable!()
    };

    let DiscoveredItem::Alias {
        alias_name: generated_alias_name,
        alias_for: generated_alias_for,
    } = generated_item
    else {
        unreachable!()
    };

    if !compare_names(expected_alias_name, generated_alias_name) {
        return false;
    }

    // Assumes correct test definition
    let expected_aliased = expected.get(expected_alias_for).unwrap();

    // We must have the aliased type in the cache
    let Some(generated_aliased) = generated.get(generated_alias_for) else {
        return false;
    };

    compare_item_info(
        expected_aliased,
        generated_aliased,
        expected,
        generated,
        expected_filename,
    )
}

pub fn compare_function_info(
    expected_item: &DiscoveredItem,
    generated_item: &DiscoveredItem,
) -> bool {
    let DiscoveredItem::Function {
        final_name: expected_final_name,
    } = expected_item
    else {
        unreachable!()
    };

    let DiscoveredItem::Function {
        final_name: generated_final_name,
    } = generated_item
    else {
        unreachable!()
    };

    if !compare_names(expected_final_name, generated_final_name) {
        return false;
    }
    true
}

pub fn compare_method_info(
    expected_item: &DiscoveredItem,
    generated_item: &DiscoveredItem,
) -> bool {
    let DiscoveredItem::Method {
        final_name: expected_final_name,
        parent: expected_parent,
    } = expected_item
    else {
        unreachable!()
    };

    let DiscoveredItem::Method {
        final_name: generated_final_name,
        parent: generated_parent,
    } = generated_item
    else {
        unreachable!()
    };

    if expected_parent != generated_parent {
        return false;
    }

    if !compare_names(expected_final_name, generated_final_name) {
        return false;
    }
    true
}

pub fn compare_mod_info(
    expected_item: &DiscoveredItem,
    generated_item: &DiscoveredItem,
) -> bool {
    let DiscoveredItem::Mod {
        final_name: expected_final_name,
        anonymous: expected_anonymous,
        inline: expected_inline,
    } = expected_item
    else {
        unreachable!()
    };

    let DiscoveredItem::Mod {
        final_name: generated_final_name,
        anonymous: generated_anonymous,
        inline: generated_inline,
    } = generated_item
    else {
        unreachable!()
    };

    if expected_anonymous != generated_anonymous
        || *expected_inline != *generated_inline
    {
        return false;
    }

    // We generate arbitrary names for anonymous namespaces - do not compare
    if !expected_anonymous {
        // Do not use regexes to compare mod names since the root mod
        // has an empty name and would match everything
        if expected_final_name != generated_final_name {
            return false;
        }
    }
    true
}
