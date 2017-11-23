extern crate bindgen;
extern crate gcc;

use std::collections::HashSet;
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use bindgen::Builder;
use bindgen::callbacks::ParseCallbacks;

#[derive(Debug)]
struct MacroCallback {
    macros: Arc<RwLock<HashSet<String>>>,
}

impl ParseCallbacks for MacroCallback {
    fn parsed_macro(&self, _name: &str) {
        self.macros.write().unwrap().insert(String::from(_name));
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
