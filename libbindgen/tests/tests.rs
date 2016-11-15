extern crate clap;
extern crate diff;
#[macro_use]
extern crate env_logger;
extern crate libbindgen;
extern crate log;
extern crate shlex;

use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::path::{Path, PathBuf};

#[path="../../src/options.rs"]
mod options;
use options::builder_from_flags;

fn compare_generated_header(header: &PathBuf,
                            builder: libbindgen::Builder)
                            -> Result<(), Error> {
    let file_name = try!(header.file_name()
        .ok_or(Error::new(ErrorKind::Other, "spawn_bindgen expects a file")));

    let mut expected = PathBuf::from(header);
    expected.pop();
    expected.pop();
    expected.push("expectations");
    expected.push("tests");
    expected.push(file_name);
    expected.set_extension("rs");

    // We skip the generate() error here so we get a full diff below
    let output = match builder.generate() {
        Ok(bindings) => bindings.to_string(),
        Err(_) => "".to_string(),
    };

    let mut buffer = String::new();
    let f = try!(fs::File::open(&expected));
    let _ = try!(BufReader::new(f).read_to_string(&mut buffer));

    if output == buffer {
        return Ok(());
    }

    println!("diff expected generated");
    println!("--- expected: {:?}", expected);
    println!("+++ generated from: {:?}", header);

    for diff in diff::lines(&buffer, &output) {
        match diff {
            diff::Result::Left(l) => println!("-{}", l),
            diff::Result::Both(l, _) => println!(" {}", l),
            diff::Result::Right(r) => println!("+{}", r),
        }
    }
    Err(Error::new(ErrorKind::Other, "Header and binding differ!"))
}

fn create_bindgen_builder(header: &PathBuf)
                          -> Result<libbindgen::Builder, Error> {
    let source = try!(fs::File::open(header));
    let reader = BufReader::new(source);

    // Scoop up bindgen-flags from test header
    let line: String = try!(reader.lines().take(1).collect());
    let flags: Vec<String> = if line.contains("bindgen-flags:") {
        line.split("bindgen-flags:").last().and_then(shlex::split)
    } else {
        None
    }.unwrap_or(Vec::with_capacity(2));

    // Fool builder_from_flags() into believing it has real env::args_os...
    // - add "bindgen" as executable name 0th element
    // - add header filename as 1st element
    // - prepend raw lines so they're in the right order for expected output
    // - append the test header's bindgen flags
    let header_str = try!(header.to_str()
        .ok_or(Error::new(ErrorKind::Other, "Invalid header file name")));

    let prepend = [
        "bindgen",
        header_str,
        "--raw-line", "",
        "--raw-line", "#![allow(non_snake_case)]",
        "--raw-line", "",
    ];

    let args = prepend.into_iter()
        .map(ToString::to_string)
        .chain(flags.into_iter());

    builder_from_flags(args).map(|(builder, _)| builder.no_unstable_rust())
}

#[test]
fn run_bindgen_tests() {
    log::set_logger(|max_log_level| {
            use env_logger::Logger;
            let env_logger = Logger::new();
            max_log_level.set(env_logger.filter());
            Box::new(env_logger)
        })
        .expect("Failed to set logger.");

    let manifest_env = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set!");
    let manifest_dir = Path::new(&manifest_env);
    let headers_dir = manifest_dir.join("tests").join("headers");

    let entries = fs::read_dir(&headers_dir)
        .expect("Couldn't read headers dir")
        .map(|result| result.expect("Couldn't read header file"));

    let headers: Vec<_> = entries.filter_map(|entry| {
            match entry.path().extension().and_then(|s| s.to_str()) {
                Some("h") | Some("hpp") => Some(entry.path()),
                _ => None,
            }
        })
        .collect();

    let failures: Vec<_> = headers.iter()
        .filter_map(|header| {
            create_bindgen_builder(header)
                .and_then(|builder| compare_generated_header(header, builder))
                .err()
        })
        .collect();

    let num_failures = failures.len();

    if num_failures > 0 {
        panic!("{} test{} failed!",
               num_failures,
               if num_failures > 1 {"s"} else {""});
    }
}
