//! Generate a module with a custom `#[path=...]` for each of the files in our
//! libclang version-specific test expectations so that they get their layout
//! tests run. We need to do this because cargo doesn't automatically detect
//! tests subdirectories.

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

const LIBCLANG_VERSION_DIRS: &'static [&'static str] =
    &["libclang-3.8", "libclang-3.9", "libclang-4"];

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let mut test_string = String::new();

    for dir in LIBCLANG_VERSION_DIRS {
        let dir = Path::new(&env::var_os("CARGO_MANIFEST_DIR").unwrap())
            .join("tests")
            .join(dir);

        println!("cargo:rerun-if-changed={}", dir.display());

        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let path = path.canonicalize().unwrap_or_else(|_| path.into());
            if path.extension().map(|e| e.to_string_lossy()) != Some("rs".into()) {
                continue;
            }

            println!("cargo:rerun-if-changed={}", path.display());

            let module_name: String = path.display()
                .to_string()
                .chars()
                .map(|c| match c {
                    'a'...'z' | 'A'...'Z' | '0'...'9' => c,
                    _ => '_',
                })
                .collect();

            test_string.push_str(&format!(
                r###"
#[path = "{}"]
mod {};
"###,
                path.display(),
                module_name,
            ));
        }
    }

    let out_path = Path::new(&env::var_os("OUT_DIR").unwrap())
        .join("libclang_version_specific_generated_tests.rs");
    let mut test_file = fs::File::create(out_path).unwrap();
    test_file.write_all(test_string.as_bytes()).unwrap();
}
