//! A library to generate __fuzzed__ C headers for use with `quickcheck`
//!
//! ## Example
//!
//! ```rust
//! extern crate quickcheck;
//! extern crate quickchecking;
//! extern crate rand;
//! use quickcheck::{Arbitrary, Gen, StdGen};
//! use quickchecking::fuzzers;
//! use rand::thread_rng;
//!
//! fn main() {
//!     let generate_range: usize = 10; // Determines things like the length of
//!                                     // arbitrary vectors generated.
//!     let header = fuzzers::HeaderC::arbitrary(
//!        &mut StdGen::new(thread_rng(), generate_range));
//!     println!("{}", header);
//! }
//! ```
//!
#![deny(missing_docs)]
#[macro_use]
extern crate lazy_static;
extern crate quickcheck;
extern crate rand;
extern crate tempdir;

use std::sync::Mutex;
use quickcheck::{QuickCheck, StdGen, TestResult};
use std::fs::File;
use std::io::Write;
use tempdir::TempDir;
use std::process::{Command, Output};
use std::path::PathBuf;
use std::error::Error;
use rand::thread_rng;

/// Contains definitions of and impls for types used to fuzz C declarations.
pub mod fuzzers;

// Global singleton, manages context across tests. For now that context is
// only the output_path for inspecting fuzzed headers (if specified).
struct Context {
    output_path: Option<String>,
}

// Initialize global context.
lazy_static! {
    static ref CONTEXT: Mutex<Context> = Mutex::new(Context { output_path: None });
}

// Passes fuzzed header to the `csmith-fuzzing/predicate.py` script, returns
// output of the associated command.
fn run_predicate_script(header: fuzzers::HeaderC) -> Result<Output, Box<Error>> {
    let dir = TempDir::new("bindgen_prop")?;
    let header_path = dir.path().join("prop_test.h");

    let mut header_file = File::create(&header_path)?;
    header_file.write_all(header.to_string().as_bytes())?;
    header_file.sync_all()?;

    let header_path_string;
    match header_path.into_os_string().into_string() {
        Ok(s) => header_path_string = s,
        Err(_) => return Err(From::from("error converting path into String")),
    }

    let mut predicate_script_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    predicate_script_path.push("../../csmith-fuzzing/predicate.py");

    let predicate_script_path_string;
    match predicate_script_path.into_os_string().into_string() {
        Ok(s) => predicate_script_path_string = s,
        Err(_) => return Err(From::from("error converting path into String")),
    }

    // Copy generated temp files to output_path directory for inspection.
    // If `None`, output path not specified, don't copy.
    match CONTEXT.lock().unwrap().output_path {
        Some(ref path) => {
            Command::new("cp")
                .arg("-a")
                .arg(&dir.path().to_str().unwrap())
                .arg(&path)
                .output()?;
        }
        None => {}
    }

    Ok(Command::new(&predicate_script_path_string)
        .arg(&header_path_string)
        .output()?)
}

// Generatable property. Pass generated headers off to run through the
// `csmith-fuzzing/predicate.py` script. Success is measured by the success
// status of that command.
fn bindgen_prop(header: fuzzers::HeaderC) -> TestResult {
    match run_predicate_script(header) {
        Ok(o) => return TestResult::from_bool(o.status.success()),
        Err(e) => {
            println!("{:?}", e);
            return TestResult::from_bool(false);
        }
    }
}

/// Instantiate a Quickcheck object and use it to run property tests using
/// fuzzed C headers generated with types defined in the `fuzzers` module.
/// Success/Failure is dictated by the result of passing the fuzzed headers
/// to the `csmith-fuzzing/predicate.py` script.
pub fn test_bindgen(generate_range: usize, tests: usize, output_path: Option<&str>) {
    match output_path {
        Some(path) => {
            CONTEXT.lock().unwrap().output_path =
                Some(String::from(PathBuf::from(path).to_str().unwrap()));
        }
        None => {} // Path not specified, don't provide output.
    }

    QuickCheck::new()
        .tests(tests)
        .gen(StdGen::new(thread_rng(), generate_range))
        .quickcheck(bindgen_prop as fn(fuzzers::HeaderC) -> TestResult)
}
