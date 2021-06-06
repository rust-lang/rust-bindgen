extern crate bindgen;
extern crate cc;

use bindgen::callbacks::{IntKind, MacroParsingBehavior, ParseCallbacks};
use bindgen::{Builder, EnumVariation};
use std::collections::HashSet;
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};

#[derive(Debug)]
struct MacroCallback {
    macros: Arc<RwLock<HashSet<String>>>,
    seen_hellos: Mutex<u32>,
    seen_funcs: Mutex<u32>,
}

impl ParseCallbacks for MacroCallback {
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        self.macros.write().unwrap().insert(name.into());

        if name == "MY_ANNOYING_MACRO" {
            return MacroParsingBehavior::Ignore;
        }

        MacroParsingBehavior::Default
    }

    fn item_name(&self, original_item_name: &str) -> Option<String> {
        if original_item_name.starts_with("my_prefixed_") {
            Some(
                original_item_name
                    .trim_start_matches("my_prefixed_")
                    .to_string(),
            )
        } else if original_item_name.starts_with("MY_PREFIXED_") {
            Some(
                original_item_name
                    .trim_start_matches("MY_PREFIXED_")
                    .to_string(),
            )
        } else {
            None
        }
    }

    fn str_macro(&self, name: &str, value: &[u8]) {
        match name {
            "TESTMACRO_STRING_EXPR" => {
                assert_eq!(value, b"string");
                *self.seen_hellos.lock().unwrap() += 1;
            }
            "TESTMACRO_STRING_EXPANDED" |
            "TESTMACRO_STRING" |
            "TESTMACRO_INTEGER" => {
                // The integer test macro is, actually, not expected to show up here at all -- but
                // should produce an error if it does.
                assert_eq!(
                    value, b"Hello Preprocessor!",
                    "str_macro handle received unexpected value"
                );
                *self.seen_hellos.lock().unwrap() += 1;
            }
            _ => {}
        }
    }

    fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
        match name {
            "TESTMACRO_CUSTOMINTKIND_PATH" => Some(IntKind::Custom {
                name: "crate::MacroInteger",
                is_signed: true,
            }),

            _ => None,
        }
    }

    fn func_macro(&self, name: &str, value: &[&[u8]]) {
        match name {
            "TESTMACRO_NONFUNCTIONAL" => {
                panic!("func_macro was called for a non-functional macro");
            }
            "TESTMACRO_FUNCTIONAL_NONEMPTY(TESTMACRO_INTEGER)" => {
                // Spaces are inserted into the right-hand side of a functional
                // macro during reconstruction from the tokenization. This might
                // change in the future, but it is safe by the definition of a
                // token in C, whereas leaving the spaces out could change
                // tokenization.
                assert_eq!(value, &[b"-" as &[u8], b"TESTMACRO_INTEGER"]);
                *self.seen_funcs.lock().unwrap() += 1;
            }
            "TESTMACRO_FUNCTIONAL_EMPTY(TESTMACRO_INTEGER)" => {
                assert_eq!(value, &[] as &[&[u8]]);
                *self.seen_funcs.lock().unwrap() += 1;
            }
            "TESTMACRO_FUNCTIONAL_TOKENIZED(a,b,c,d,e)" => {
                assert_eq!(
                    value,
                    &[b"a" as &[u8], b"/", b"b", b"c", b"d", b"##", b"e"]
                );
                *self.seen_funcs.lock().unwrap() += 1;
            }
            "TESTMACRO_FUNCTIONAL_SPLIT(a,b)" => {
                assert_eq!(value, &[b"b", b",", b"a"]);
                *self.seen_funcs.lock().unwrap() += 1;
            }
            "TESTMACRO_STRING_FUNC_NON_UTF8(x)" => {
                assert_eq!(
                    value,
                    &[b"(" as &[u8], b"x", b"\"\xff\xff\"", b")"]
                );
                *self.seen_funcs.lock().unwrap() += 1;
            }
            _ => {
                // The system might provide lots of functional macros.
                // Ensure we did not miss handling one that we meant to handle.
                assert!(!name.starts_with("TESTMACRO_"), "name = {}", name);
            }
        }
    }

    // Test the "custom derives" capability by adding `PartialEq` to the `Test` struct.
    fn add_derives(&self, name: &str) -> Vec<String> {
        if name == "Test" {
            vec![
                "PartialEq".into(),
            ]
        } else {
            vec![]
        }
    }
}

impl Drop for MacroCallback {
    fn drop(&mut self) {
        assert_eq!(
            *self.seen_hellos.lock().unwrap(),
            3,
            "str_macro handle was not called once for all relevant macros"
        );
        assert_eq!(
            *self.seen_funcs.lock().unwrap(),
            5,
            "func_macro handle was not called once for all relevant macros"
        );
    }
}

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("cpp/Test.cc")
        .include("include")
        .compile("libtest.a");

    let macros = Arc::new(RwLock::new(HashSet::new()));

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_rust_file = out_path.join("test.rs");
    let out_rust_file_relative = out_rust_file.strip_prefix(std::env::current_dir().unwrap()).unwrap();
    let out_dep_file = out_path.join("test.d");

    let bindings = Builder::default()
        .rustfmt_bindings(false)
        .enable_cxx_namespaces()
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: false,
        })
        .raw_line("pub use self::root::*;")
        .raw_line("extern { fn my_prefixed_function_to_remove(i: i32); }")
        .module_raw_line("root::testing", "pub type Bar = i32;")
        .header("cpp/Test.h")
        .clang_args(&["-x", "c++", "-std=c++11", "-I", "include"])
        .parse_callbacks(Box::new(MacroCallback {
            macros: macros.clone(),
            seen_hellos: Mutex::new(0),
            seen_funcs: Mutex::new(0),
        }))
        .blocklist_function("my_prefixed_function_to_remove")
        .constified_enum("my_prefixed_enum_to_be_constified")
        .opaque_type("my_prefixed_templated_foo<my_prefixed_baz>")
        .depfile(out_rust_file_relative.display().to_string(), &out_dep_file)
        .generate()
        .expect("Unable to generate bindings");

    assert!(macros.read().unwrap().contains("TESTMACRO"));
    bindings.write_to_file(&out_rust_file).expect("Couldn't write bindings!");

    let observed_deps = std::fs::read_to_string(out_dep_file).expect("Couldn't read depfile!");
    let expected_deps = format!("{}: cpp/Test.h include/stub.h", out_rust_file_relative.display());
    assert_eq!(
        observed_deps,
        expected_deps,
        "including stub via include dir must produce correct dep path",
    );
}
