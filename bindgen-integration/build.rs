extern crate bindgen;
extern crate gcc;

use std::env;
use std::path::PathBuf;
use bindgen::Builder;

fn main() {
    gcc::Config::new()
        .cpp(true)
        .file("cpp/Test.cc")
        .compile("libtest.a");

    let bindings = Builder::default()
        .no_unstable_rust()
        .enable_cxx_namespaces()
        .raw_line("pub use self::root::*;")
        .header("cpp/Test.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        .generate()
        .expect("Unable to generate bindings");


    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("test.rs"))
        .expect("Couldn't write bindings!");
}
