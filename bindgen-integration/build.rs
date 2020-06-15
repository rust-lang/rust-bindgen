extern crate bindgen;
extern crate cc;

use bindgen::callbacks::{MacroParsingBehavior, ParseCallbacks, IntKind};
use bindgen::Builder;
use std::collections::HashSet;
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};

#[derive(Debug)]
struct MacroCallback {
    macros: Arc<RwLock<HashSet<String>>>,
    seen_hellos: Mutex<u32>,
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
}

impl Drop for MacroCallback {
    fn drop(&mut self) {
        assert_eq!(
            *self.seen_hellos.lock().unwrap(),
            2,
            "str_macro handle was not called once for all relevant macros"
        )
    }
}

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("cpp/Test.cc")
        .compile("libtest.a");

    let macros = Arc::new(RwLock::new(HashSet::new()));

    let bindings = Builder::default()
        .rustfmt_bindings(false)
        .enable_cxx_namespaces()
        .rustified_enum(".*")
        .raw_line("pub use self::root::*;")
        .raw_line("extern { fn my_prefixed_function_to_remove(i: i32); }")
        .module_raw_line("root::testing", "pub type Bar = i32;")
        .header("cpp/Test.h")
        .clang_args(&["-x", "c++", "-std=c++11"])
        .parse_callbacks(Box::new(MacroCallback {
            macros: macros.clone(),
            seen_hellos: Mutex::new(0),
        }))
        .blacklist_function("my_prefixed_function_to_remove")
        .generate()
        .expect("Unable to generate bindings");

    assert!(macros.read().unwrap().contains("TESTMACRO"));

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("test.rs"))
        .expect("Couldn't write bindings!");
}
