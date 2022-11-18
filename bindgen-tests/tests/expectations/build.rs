//! Generate a module with a custom `#[path=...]` for each of the files in our
//! libclang version-specific test expectations so that they get their layout
//! tests run. We need to do this because cargo doesn't automatically detect
//! tests subdirectories.

use std::env;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::process::Command;

const LIBCLANG_VERSION_DIRS: &[&str] = &["libclang-5", "libclang-9", ""];

#[derive(Clone)]
enum Version {
    Nightly,
    Stable([u8; 3]),
}

impl Version {
    fn new(s: &str) -> Self {
        if s == "nightly" {
            Version::Nightly
        } else {
            let mut version = [0; 3];

            let mut parts = s.split('.').map(|s| s.parse::<u8>().unwrap());

            for i in 0..3 {
                version[i] = parts.next().unwrap_or_default();
            }

            Version::Stable(version)
        }
    }

    fn is_compatible_with(&self, other: &Version) -> bool {
        match (self, other) {
            (Version::Nightly, Version::Nightly) => true,
            (Version::Nightly, Version::Stable(_)) => true,
            (Version::Stable(_), Version::Nightly) => false,
            (Version::Stable(v_self), Version::Stable(v_other)) => {
                v_other <= v_self
            }
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let rustc_version = {
        let rustc =
            env::var("RUSTC").expect("`RUSTC` environment variable is not set");
        let cmd_output = Command::new(rustc)
            .arg("--version")
            .output()
            .expect("Couldn't run `rustc --version`")
            .stdout;
        Version::new(
            &String::from_utf8(
                cmd_output
                    .split(|b| b.is_ascii_whitespace())
                    .skip(1)
                    .next()
                    .unwrap()
                    .to_owned(),
            )
            .unwrap(),
        )
    };

    let mut test_string = String::new();

    for dir in LIBCLANG_VERSION_DIRS {
        let dir_path = Path::new(&env::var_os("CARGO_MANIFEST_DIR").unwrap())
            .join("bindings")
            .join(dir);

        println!("cargo:rerun-if-changed={}", dir_path.display());

        for entry in fs::read_dir(dir_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let path = path.canonicalize().unwrap_or(path);
            if path.extension().map(|e| e.to_string_lossy()) !=
                Some("rs".into())
            {
                continue;
            }

            println!("cargo:rerun-if-changed={}", path.display());

            let stem = path.file_stem();

            for entry in fs::read_dir("../headers/").unwrap() {
                let entry = entry.unwrap();
                let headers_path = entry.path();
                let headers_path =
                    headers_path.canonicalize().unwrap_or(headers_path);
                if headers_path.file_stem() == stem && headers_path.is_file() {
                    let mut target_version = None;
                    let file =
                        BufReader::new(File::open(&headers_path).unwrap());
                    for line in file.lines() {
                        let line = line.unwrap();
                        if line.starts_with("// bindgen-flags: ") {
                            target_version = line
                                .split("--rust-target")
                                .skip(1)
                                .next()
                                .and_then(|s| s[1..].split(' ').next())
                                .map(Version::new);
                            break;
                        }
                    }
                    let target_version = target_version.unwrap_or_else(|| rustc_version.clone());
                    if rustc_version.is_compatible_with(&target_version) {
                        let module_name: String = stem
                            .join(dir)
                            .display()
                            .to_string()
                            .chars()
                            .map(|c| match c {
                                'a'..='z' | 'A'..='Z' | '0'..='9' => c,
                                _ => '_',
                            })
                            .collect();

                        test_string.push_str(&format!(
                            r###"
#[path = "{}"]
mod {};
"###,
                            path.display().to_string().replace('\\', "\\\\"),
                            module_name.trim_matches('_'),
                        ));
                    }

                    break;
                }
            }
        }
    }

    let out_path = Path::new(&env::var_os("OUT_DIR").unwrap())
        .join("libclang_version_specific_generated_tests.rs");
    let mut test_file = fs::File::create(out_path).unwrap();
    test_file.write_all(test_string.as_bytes()).unwrap();
}
