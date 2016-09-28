mod codegen {
    extern crate quasi_codegen;
    use std::path::Path;
    use std::env;

    pub fn main() {
        let out_dir = env::var_os("OUT_DIR").unwrap();
        let src = Path::new("src/codegen/mod.rs");
        let dst = Path::new(&out_dir).join("codegen.rs");

        quasi_codegen::expand(&src, &dst).unwrap();
        println!("cargo:rerun-if-changed=src/codegen/mod.rs");
        println!("cargo:rerun-if-changed=src/codegen/helpers.rs");
    }
}

fn main() {
    use std::env;

    codegen::main();
    if let Ok(path) = env::var("LIBCLANG_PATH") {
        println!("cargo:rustc-link-search=native={}", path);
    }
}
