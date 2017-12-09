//! An application to run property tests for `bindgen` with _fuzzed_ C headers
//! using `quickcheck`
//!
//! ## Usage
//!
//! Print help
//! ```bash
//! $ cargo run --bin=quickchecking -- -h
//! ```
//!
//! Run with default values
//! ```bash
//! $ cargo run --bin=quickchecking
//! ```
//!
#![deny(missing_docs)]
extern crate clap;
extern crate quickchecking;

use clap::{App, Arg};
use std::path::Path;

// Validate CLI argument input for generation range.
fn validate_generate_range(v: String) -> Result<(), String> {
    match v.parse::<usize>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from(
            "Generate range could not be converted to a usize.",
        )),
    }
}

// Validate CLI argument input for tests count.
fn validate_tests_count(v: String) -> Result<(), String> {
    match v.parse::<usize>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from(
            "Tests count could not be converted to a usize.",
        )),
    }
}

// Validate CLI argument input for fuzzed headers output path.
fn validate_path(v: String) -> Result<(), String> {
    match Path::new(&v).is_dir() {
        true => Ok(()),
        false => Err(String::from("Provided directory path does not exist.")),
    }
}

fn main() {
    let matches = App::new("quickchecking")
        .version("0.2.0")
        .about(
            "Bindgen property tests with quickcheck. \
             Generate random valid C code and pass it to the \
             csmith/predicate.py script",
        )
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .value_name("PATH")
                .help(
                    "Optional. Preserve generated headers for inspection, \
                     provide directory path for header output. [default: None] ",
                )
                .takes_value(true)
                .validator(validate_path),
        )
        .arg(
            Arg::with_name("range")
                .short("r")
                .long("range")
                .value_name("RANGE")
                .help(
                    "Sets the range quickcheck uses during generation. \
                     Corresponds to things like arbitrary usize and \
                     arbitrary vector length. This number doesn't have \
                     to grow much for execution time to increase \
                     significantly.",
                )
                .takes_value(true)
                .default_value("32")
                .validator(validate_generate_range),
        )
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .value_name("COUNT")
                .help(
                    "Count / number of tests to run. Running a fuzzed \
                     header through the predicate.py script can take a \
                     long time, especially if the generation range is \
                     large. Increase this number if you're willing to \
                     wait a while.",
                )
                .takes_value(true)
                .default_value("2")
                .validator(validate_tests_count),
        )
        .get_matches();

    let output_path: Option<&str> = matches.value_of("path");
    let generate_range: usize = matches.value_of("range").unwrap().parse::<usize>().unwrap();
    let tests: usize = matches.value_of("count").unwrap().parse::<usize>().unwrap();

    quickchecking::test_bindgen(generate_range, tests, output_path)
}
