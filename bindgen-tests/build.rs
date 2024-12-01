use std::char;
use std::env;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut dst = File::create(Path::new(&out_dir).join("tests.rs")).unwrap();

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let headers_dir = manifest_dir.join("tests").join("headers");

    let Ok(headers) = fs::read_dir(headers_dir) else {
        // We may not have headers directory after packaging.
        return;
    };

    let entries =
        headers.map(|result| result.expect("Couldn't read header file"));

    println!("cargo:rerun-if-changed=tests/headers");

    for entry in entries {
        match entry.path().extension().and_then(OsStr::to_str) {
            Some("h") | Some("hpp") => {
                let func = entry
                    .file_name()
                    .to_str()
                    .unwrap()
                    .replace(|c| !char::is_alphanumeric(c), "_")
                    .replace("__", "_")
                    .to_lowercase();
                writeln!(
                    dst,
                    "test_header!(header_{func}, {:?});",
                    entry.path(),
                )
                .unwrap();
            }
            _ => {}
        }
    }

    dst.flush().unwrap();
}
