extern crate quickcheck;
extern crate quickchecking;
extern crate rand;
extern crate tempdir;

use quickchecking::fuzzers;
use quickcheck::{QuickCheck, StdGen, TestResult};
use std::fs::File;
use std::io::Write;
use tempdir::TempDir;
use std::process::{Command, Output};
use std::path::PathBuf;
use std::error::Error;
use rand::thread_rng;

fn run_predicate_script(header: fuzzers::HeaderC, header_name: &str) -> Result<Output, Box<Error>> {
    let dir = TempDir::new("bindgen_prop")?;
    let header_path = dir.path().join(header_name);

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

    // Copy generated temp files to test directory for inspection.
    // Preserved for anyone interested in validating the behavior.

    let mut debug_output_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    debug_output_path.push("tests");
    Command::new("cp")
        .arg("-a")
        .arg(&dir.path().to_str().unwrap())
        .arg(&debug_output_path.to_str().unwrap())
        .output()?;

    Ok(Command::new(&predicate_script_path_string)
        .arg(&header_path_string)
        .output()?)
}

fn bindgen_prop(header: fuzzers::HeaderC) -> TestResult {
    match run_predicate_script(header, "prop_test.h") {
        Ok(o) => return TestResult::from_bool(o.status.success()),
        Err(e) => {
            println!("{:?}", e);
            return TestResult::from_bool(false);
        }
    }
}

#[test]
fn test_bindgen() {
    // Enough to generate any value in the PrimitiveTypeC `base_type` list.
    let generate_range: usize = 32;
    QuickCheck::new()
        // Generating is relatively quick (generate_range 150 takes ~5 seconds)
        // but running predicate.py takes ~30 seconds per source file / test
        // when the generation range is just 32. It can take a lot longer with a
        // higher generate_range. Up the number of tests or generate_range if
        // you're willing to wait awhile.
        .tests(2)
        .gen(StdGen::new(thread_rng(), generate_range))
        .quickcheck(bindgen_prop as fn(fuzzers::HeaderC) -> TestResult)
}
