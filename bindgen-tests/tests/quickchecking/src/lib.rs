//! A library to generate __fuzzed__ C headers for use with `quickcheck`
//!
//! ## Example
//!
//! ```rust
//! use quickcheck::{Arbitrary, Gen};
//! use quickchecking::fuzzers;
//!
//! fn main() {
//!     let generate_range: usize = 10; // Determines things like the length of
//!                                     // arbitrary vectors generated.
//!     let header = fuzzers::HeaderC::arbitrary(
//!        &mut Gen::new(generate_range));
//!     println!("{}", header);
//! }
//! ```
#![deny(missing_docs)]

use quickcheck::{Gen, QuickCheck, TestResult};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::Mutex;
use tempfile::Builder;

/// Contains definitions of and impls for types used to fuzz C declarations.
pub mod fuzzers;

// Global singleton, manages context across tests. For now that context is
// only the output_path for inspecting fuzzed headers (if specified).
struct Context {
    output_path: Option<String>,
}

// Initialize global context.
static CONTEXT: Mutex<Context> = Mutex::new(Context { output_path: None });

// Passes fuzzed header to the `csmith-fuzzing/predicate.py` script, returns
// output of the associated command.
fn run_predicate_script(
    header: fuzzers::HeaderC,
) -> Result<Output, Box<dyn Error>> {
    let dir = Builder::new().prefix("bindgen_prop").tempdir()?;
    let header_path = dir.path().join("prop_test.h");

    let mut header_file = File::create(&header_path)?;
    header_file.write_all(header.to_string().as_bytes())?;
    header_file.sync_all()?;

    let header_path_string = header_path
        .into_os_string()
        .into_string()
        .map_err(|_| "error converting path into String")?;

    let mut predicate_script_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    predicate_script_path.push("../../csmith-fuzzing/predicate.py");

    let predicate_script_path_string = predicate_script_path
        .into_os_string()
        .into_string()
        .map_err(|_| "error converting path into String")?;

    // Copy generated temp files to output_path directory for inspection.
    // If `None`, output path not specified, don't copy.
    if let Some(ref path) = CONTEXT.lock().unwrap().output_path {
        Command::new("cp")
            .arg("-a")
            .arg(dir.path().to_str().unwrap())
            .arg(path)
            .output()?;
    }

    Ok(Command::new(predicate_script_path_string)
        .arg(&header_path_string)
        .output()?)
}

// Generatable property. Pass generated headers off to run through the
// `csmith-fuzzing/predicate.py` script. Success is measured by the success
// status of that command.
fn bindgen_prop(header: fuzzers::HeaderC) -> TestResult {
    match run_predicate_script(header) {
        Ok(o) => TestResult::from_bool(o.status.success()),
        Err(e) => {
            println!("{:?}", e);
            TestResult::from_bool(false)
        }
    }
}

/// Instantiate a Quickcheck object and use it to run property tests using
/// fuzzed C headers generated with types defined in the `fuzzers` module.
/// Success/Failure is dictated by the result of passing the fuzzed headers
/// to the `csmith-fuzzing/predicate.py` script.
pub fn test_bindgen(
    generate_range: usize,
    tests: u64,
    output_path: Option<&Path>,
) {
    if let Some(path) = output_path {
        CONTEXT.lock().unwrap().output_path = Some(path.display().to_string());
    }

    QuickCheck::new()
        .tests(tests)
        .gen(Gen::new(generate_range))
        .quickcheck(bindgen_prop as fn(fuzzers::HeaderC) -> TestResult)
}
