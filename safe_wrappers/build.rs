extern crate bindgen;

use std::env;
use std::process::Command;
use std::io::Write;

fn main() {
    println!("cargo:rerun-if-changed=example.cpp");
    println!("cargo:rustc-link-lib=bindexample");
    println!("cargo:rustc-link-lib=stdc++");

    {
        let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .truncate(true)
        .open("generated.cpp")
        .unwrap();

        file.write_all(b"#include \"example.cpp\"\n").expect("unable to write");
    }

	let _ = bindgen::Builder::default()
        .header("example.cpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg("-x")
        .clang_arg("c++")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&format!("{}{}", env::var("OUT_DIR").unwrap(), "/bindings_1.rs"))
        .expect("Couldn't write bindings!");

    let status = Command::new("cc")
        .args(&["generated.cpp", "-shared", "-fPIC", "-o", &format!("{}{}", env::var("OUT_DIR").unwrap(), "/../../../deps/libbindexample.so")])
        .status()
        .expect("failed to execute process");
    assert!(status.success());

    let _ = bindgen::Builder::default()
         .header("generated.cpp")
         .parse_callbacks(Box::new(bindgen::CargoCallbacks))
         .clang_arg("-x")
         .clang_arg("c++")
         .generate()
         .expect("Unable to generate bindings")
         .write_to_file(&format!("{}{}", env::var("OUT_DIR").unwrap(), "/bindings_2.rs"))
         .expect("Couldn't write bindings!");
}