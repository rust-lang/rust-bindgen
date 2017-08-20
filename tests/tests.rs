extern crate clap;
extern crate diff;
extern crate bindgen;
extern crate shlex;

use bindgen::{Builder, builder, clang_version};
use std::fs;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read, Write};
use std::path::PathBuf;

#[path = "../src/options.rs"]
mod options;
use options::builder_from_flags;

fn compare_generated_header(
    header: &PathBuf,
    builder: Builder,
) -> Result<(), Error> {
    let file_name = try!(header.file_name().ok_or(Error::new(
        ErrorKind::Other,
        "spawn_bindgen expects a file",
    )));

    let mut expected = PathBuf::from(header);
    expected.pop();
    expected.pop();
    expected.push("expectations");
    expected.push("tests");
    expected.push(file_name);
    expected.set_extension("rs");

    // If the expectation file doesn't exist, see if we have different test
    // expectations for different libclang versions.
    if !expected.is_file() {
        let file_name = expected.file_name().unwrap().to_owned();
        expected.pop();

        if cfg!(feature = "testing_only_libclang_4") {
            expected.push("libclang-4");
        } else if cfg!(feature = "testing_only_libclang_3_9") {
            expected.push("libclang-3.9");
        } else if cfg!(feature = "testing_only_libclang_3_8") {
            expected.push("libclang-3.8");
        } else {
            match clang_version().parsed {
                None => {}
                Some(version) => {
                    let (maj, min) = version;
                    let version_str = if maj >= 4 {
                        "4".to_owned()
                    } else {
                        format!("{}.{}", maj, min)
                    };
                    expected.push(format!("libclang-{}", version_str));
                }
            }
        }

        expected.push(file_name);

        if !expected.is_file() {
            panic!(
                "missing test expectation file and/or 'testing_only_libclang_$VERSION' \
                    feature for header '{}'; looking for expectation file at '{}'",
                header.display(),
                expected.display()
            );
        }
    }

    // We skip the generate() error here so we get a full diff below
    let output = match builder.generate() {
        Ok(bindings) => bindings.to_string(),
        Err(_) => "".to_string(),
    };

    let mut buffer = String::new();
    {
        if let Ok(expected_file) = fs::File::open(&expected) {
            try!(BufReader::new(expected_file).read_to_string(&mut buffer));
        }
    }

    if output == buffer {
        if !output.is_empty() {
            return Ok(());
        }
        return Err(Error::new(
            ErrorKind::Other,
            "Something's gone really wrong!",
        ));
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

    // Override the diff.
    {
        let mut expected_file = try!(fs::File::create(&expected));
        try!(expected_file.write_all(output.as_bytes()));
    }

    Err(Error::new(ErrorKind::Other, "Header and binding differ!"))
}

fn create_bindgen_builder(header: &PathBuf) -> Result<Option<Builder>, Error> {
    let source = try!(fs::File::open(header));
    let reader = BufReader::new(source);

    // Scoop up bindgen-flags from test header
    let mut flags = Vec::with_capacity(2);

    for line in reader.lines() {
        let line = try!(line);
        if !line.starts_with("// bindgen") {
            continue;
        }

        if line.contains("bindgen-flags: ") {
            let extra_flags = line.split("bindgen-flags: ")
                .last()
                .and_then(shlex::split)
                .unwrap();
            flags.extend(extra_flags.into_iter());
        } else if line.contains("bindgen-osx-only") {
            let prepend_flags = ["--raw-line", "#![cfg(target_os=\"macos\")]"];
            flags = prepend_flags
                .into_iter()
                .map(ToString::to_string)
                .chain(flags)
                .collect();
        } else if line.contains("bindgen-generate-bindings-on-linux-only") &&
                   !cfg!(target_os = "linux")
        {
            return Ok(None);
        }
    }

    // Windows platform has various different conventions than *nix platforms,
    // e.g. default enum underlying type, struct padding, mangling. Most tests
    // were written and checked on Linux and macOS, and thus they could fail on
    // Windows. We just make those tests targetting Linux instead as far as one
    // isn't annotated for a specific target.
    if cfg!(target_os = "windows") {
        if flags.iter().all(|flag| !flag.starts_with("--target=")) {
            if !flags.iter().any(|flag| flag == "--") {
                flags.push("--".into());
            }
            flags.push("--target=x86_64-unknown-linux".into());
        }
    }

    // Fool builder_from_flags() into believing it has real env::args_os...
    // - add "bindgen" as executable name 0th element
    // - add header filename as 1st element
    // - prepend raw lines so they're in the right order for expected output
    // - append the test header's bindgen flags
    let header_str = try!(header.to_str().ok_or(Error::new(
        ErrorKind::Other,
        "Invalid header file name",
    )));

    let prepend = [
        "bindgen",
        "--with-derive-default",
        header_str,
        "--raw-line",
        "",
        "--raw-line",
        "#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]",
        "--raw-line",
        "",
    ];

    let args = prepend.into_iter().map(ToString::to_string).chain(
        flags
            .into_iter(),
    );

    builder_from_flags(args).map(|(builder, _, _)| Some(builder))
}

macro_rules! test_header {
    ($function:ident, $header:expr) => (
        #[test]
        fn $function() {
            let header = PathBuf::from($header);
            let result = create_bindgen_builder(&header)
                .and_then(|builder| {
                    if let Some(builder) = builder {
                        compare_generated_header(&header, builder)
                    } else {
                        Ok(())
                    }
                });

            if let Err(err) = result {
                panic!("{}", err);
            }
        }
    )
}

// This file is generated by build.rs
include!(concat!(env!("OUT_DIR"), "/tests.rs"));

#[test]
fn test_header_contents() {
    let bindings = builder()
        .header_contents("test.h", "int foo(const char* a);")
        .generate()
        .unwrap()
        .to_string();
    assert_eq!(
        bindings,
        "/* automatically generated by rust-bindgen */

extern \"C\" {
    pub fn foo(a: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
"
    );
}

#[test]
fn test_multiple_header_calls_in_builder() {
    let actual = builder()
        .header(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/headers/func_ptr.h"
        ))
        .header(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/headers/char.h"))
        .generate()
        .unwrap()
        .to_string();

    let expected = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/expectations/tests/test_multiple_header_calls_in_builder.rs"
    ));

    if actual != expected {
        println!("Generated bindings differ from expected!");

        for diff in diff::lines(&actual, &expected) {
            match diff {
                diff::Result::Left(l) => println!("-{}", l),
                diff::Result::Both(l, _) => println!(" {}", l),
                diff::Result::Right(r) => println!("+{}", r),
            }
        }

        panic!();
    }
}

#[test]
// Doesn't support executing sh file on Windows.
// We may want to implement it in Rust so that we support all systems.
#[cfg(not(target_os = "windows"))]
fn no_system_header_includes() {
    use std::process::Command;
    assert!(
        Command::new("./ci/no-includes.sh")
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .spawn()
            .expect("should spawn ./ci/no-includes.sh OK")
            .wait()
            .expect("should wait for ./ci/no-includes OK")
            .success()
    );
}

#[test]
fn dump_preprocessed_input() {
    let arg_keyword =
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/headers/arg_keyword.hpp");
    let empty_layout = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/headers/cpp-empty-layout.hpp"
    );

    builder()
        .header(arg_keyword)
        .header(empty_layout)
        .dump_preprocessed_input()
        .expect("should dump preprocessed input");

    fn slurp(p: &str) -> String {
        let mut contents = String::new();
        let mut file = fs::File::open(p).unwrap();
        file.read_to_string(&mut contents).unwrap();
        contents
    }

    let bindgen_ii = slurp("__bindgen.ii");
    let arg_keyword = slurp(arg_keyword);
    let empty_layout = slurp(empty_layout);

    assert!(
        bindgen_ii.find(&arg_keyword).is_some(),
        "arg_keyword.hpp is in the preprocessed file"
    );
    assert!(
        bindgen_ii.find(&empty_layout).is_some(),
        "cpp-empty-layout.hpp is in the preprocessed file"
    );
}
