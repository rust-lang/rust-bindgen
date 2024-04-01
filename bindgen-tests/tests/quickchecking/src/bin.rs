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

use clap::{Arg, ArgAction, Command};
use std::path::PathBuf;

// Parse CLI argument input for generation range.
fn parse_generate_range(v: &str) -> Result<usize, String> {
    match v.parse::<usize>() {
        Ok(v) => Ok(v),
        Err(_) => Err(String::from(
            "Generate range could not be converted to a usize.",
        )),
    }
}

// Parse CLI argument input for tests count.
fn parse_tests_count(v: &str) -> Result<u64, String> {
    match v.parse::<u64>() {
        Ok(v) => Ok(v),
        Err(_) => Err(String::from(
            "Tests count could not be converted to a usize.",
        )),
    }
}

// Parse CLI argument input for fuzzed headers output path.
fn parse_path(v: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(v);
    match path.is_dir() {
        true => Ok(path),
        false => Err(String::from("Provided directory path does not exist.")),
    }
}

fn main() {
    let matches = Command::new("quickchecking")
        .version("0.2.0")
        .about(
            "Bindgen property tests with quickcheck. \
             Generate random valid C code and pass it to the \
             csmith/predicate.py script",
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .help(
                    "Optional. Preserve generated headers for inspection, \
                     provide directory path for header output. [default: None] ",
                )
                .action(ArgAction::Set)
                .value_parser(parse_path),
        )
        .arg(
            Arg::new("range")
                .short('r')
                .long("range")
                .value_name("RANGE")
                .help(
                    "Sets the range quickcheck uses during generation. \
                     Corresponds to things like arbitrary usize and \
                     arbitrary vector length. This number doesn't have \
                     to grow much for execution time to increase \
                     significantly.",
                )
                .action(ArgAction::Set)
                .default_value("32")
                .value_parser(parse_generate_range),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .value_name("COUNT")
                .help(
                    "Count / number of tests to run. Running a fuzzed \
                     header through the predicate.py script can take a \
                     long time, especially if the generation range is \
                     large. Increase this number if you're willing to \
                     wait a while.",
                )
                .action(ArgAction::Set)
                .default_value("2")
                .value_parser(parse_tests_count),
        )
        .get_matches();

    let output_path = matches.get_one::<PathBuf>("path").map(PathBuf::as_path);
    let generate_range = *matches.get_one::<usize>("range").unwrap();
    let tests = *matches.get_one::<u64>("count").unwrap();

    quickchecking::test_bindgen(generate_range, tests, output_path)
}
