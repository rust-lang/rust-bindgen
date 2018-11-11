extern crate bindgen;
extern crate gcc;

use std::collections::HashSet;
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use bindgen::Builder;
use bindgen::callbacks::{MacroParsingBehavior, ParseCallbacks};

#[derive(Debug)]
struct MacroCallback {
    macros: Arc<RwLock<HashSet<String>>>,
}

impl ParseCallbacks for MacroCallback {
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        self.macros.write().unwrap().insert(name.into());

        if name == "MY_ANNOYING_MACRO" {
            return MacroParsingBehavior::Ignore
        }

        MacroParsingBehavior::Default
    }

    fn item_name(&self, original_item_name: &str) -> Option<String> {
        if original_item_name.starts_with("my_prefixed_") {
            Some(original_item_name.trim_start_matches("my_prefixed_").to_string())
        } else if original_item_name.starts_with("MY_PREFIXED_") {
            Some(original_item_name.trim_start_matches("MY_PREFIXED_").to_string())
        } else {
            None
        }
    }
}

fn main() {
    gcc::Build::new()
        .cpp(true)
        .file("cpp/Test.cc")
        .compile("libtest.a");

    let macros = Arc::new(RwLock::new(HashSet::new()));

    let bindings = Builder::default()
        .rustfmt_bindings(false)
        .enable_cxx_namespaces()
        .rustified_enum(".*")
        .raw_line("pub use self::root::*;")
        .module_raw_line("root::testing", "pub type Bar = i32;")
        .header("cpp/Test.h")
        .clang_args(&["-x", "c++", "-std=c++11"])
        .parse_callbacks(Box::new(MacroCallback {macros: macros.clone()}))
        .generate()
        .expect("Unable to generate bindings");

    assert!(macros.read().unwrap().contains("TESTMACRO"));

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("test.rs"))
        .expect("Couldn't write bindings!");
}
