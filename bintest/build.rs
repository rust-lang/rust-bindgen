extern crate bindgen;

use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=example.cpp");
    println!("cargo:rustc-link-lib=bindexample");

    let output = Command::new("cc")
            .args(&["example.cpp", "-shared", "-o", &format!("{}{}", env::var("OUT_DIR").unwrap(), "/../../../deps/libbindexample.so")])
            .output()
            .expect("failed to execute process");
    assert!(output.status.success(), "C++ compilation failed");

    let _bindings = bindgen::Builder::default()
        .header("example.cpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg("-x")
        .clang_arg("c++")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&format!("{}{}", env::var("OUT_DIR").unwrap(), "/bindings.rs"))
        .expect("Couldn't write bindings!");
}