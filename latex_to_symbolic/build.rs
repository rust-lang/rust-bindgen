extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
	let profile = env::var("PROFILE").unwrap();

	if profile == "debug" {
		let output = Command::new("cp")
			.args(&["/usr/local/lib/libginac.so.6", "target/debug/deps/libginac.so.6"])
			.output()
			.expect("failed to execute process");
		assert!(output.status.success());
		let output = Command::new("g++")
			.args(&["wrapper.cpp", "-lginac", "-fPIC", "-shared", "-o", "target/debug/deps/libwrap.so"])
			.output()
			.expect("failed to execute process");
		assert!(output.status.success());
	} else if profile == "release" {
		let output = Command::new("cp")
			.args(&["/usr/local/lib/libginac.so.6", "target/release/deps/libginac.so.6"])
			.output()
			.expect("failed to execute process");
		assert!(output.status.success());
		let output = Command::new("g++")
			.args(&["wrapper.cpp", "-lginac", "-O3", "-fPIC", "-shared", "-o", "target/release/deps/libwrap.so"])
			.output()
			.expect("failed to execute process");
		assert!(output.status.success());
	} else {
		panic!("unreachable");
	}

	//g++ wrapper.cpp -lginac -fPIC -shared -o target/debug/deps/libwrap.so


    println!("cargo:rustc-link-lib=ginac");
	println!("cargo:rustc-link-lib=wrap");

	println!("cargo:rerun-if-changed=wrapper.hpp");
    println!("cargo:rerun-if-changed=wrapper.cpp");
	println!("cargo:rerun-if-changed=convert.py");

	//let wl = "MySpace.*";
	let wl1 = "GiNaC::.*";
	let wl2 = "wrap.*";
    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg("-x")
        .clang_arg("c++")
        .opaque_type("std::.*")

		.whitelist_var(wl1)
        .whitelist_type(wl1)
        .whitelist_function(wl1)

		.whitelist_var(wl2)
        .whitelist_type(wl2)
		.whitelist_function(wl2)
		
		.gen_safe_wrappers(true)

        .generate()
        .expect("Unable to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

	let output = Command::new("python")
        .args(&["convert.py"])
        .output()
        .expect("failed to execute process");
	assert!(output.status.success());
}