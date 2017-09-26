extern crate clap;
extern crate diff;
extern crate bindgen;
extern crate shlex;

use bindgen::{Builder, builder, clang_version};
use std::fs;
use std::io::{self, BufRead, BufReader, Error, ErrorKind, Read, Write};
use std::path::PathBuf;
use std::process;
use std::sync::{Once, ONCE_INIT};

#[path = "../src/options.rs"]
mod options;
use options::builder_from_flags;

// Run `rustfmt` on the given source string and return a tuple of the formatted
// bindings, and rustfmt's stderr.
fn rustfmt(source: String) -> (String, String) {
    static CHECK_RUSTFMT: Once = ONCE_INIT;

    CHECK_RUSTFMT.call_once(|| {
        let have_working_rustfmt = process::Command::new("rustup")
            .args(&["run", "nightly", "rustfmt", "--version"])
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .status()
            .ok()
            .map_or(false, |status| status.success());

        if !have_working_rustfmt {
            panic!("
The latest `rustfmt` is required to run the `bindgen` test suite. Install
`rustfmt` with:

    $ rustup update nightly
    $ rustup run nightly cargo install -f rustfmt-nightly
");
        }
    });

    let mut child = process::Command::new("rustup")
        .args(&[
            "run",
            "nightly",
            "rustfmt",
            "--config-path",
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/rustfmt.toml")
        ])
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn()
        .expect("should spawn `rustup run nightly rustfmt`");

    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();

    // Write to stdin in a new thread, so that we can read from stdout on this
    // thread. This keeps the child from blocking on writing to its stdout which
    // might block us from writing to its stdin.
    let stdin_handle = ::std::thread::spawn(move || {
        stdin.write_all(source.as_bytes())
    });

    // Read stderr on a new thread for similar reasons.
    let stderr_handle = ::std::thread::spawn(move || {
        let mut output = vec![];
        io::copy(&mut stderr, &mut output)
            .map(|_| String::from_utf8_lossy(&output).to_string())
    });

    let mut output = vec![];
    io::copy(&mut stdout, &mut output)
        .expect("Should copy stdout into vec OK");

    // Ignore actual rustfmt status because it is often non-zero for trivial
    // things.
    let _ = child.wait().expect("should wait on rustfmt child OK");

    stdin_handle.join()
        .expect("writer thread should not have panicked")
        .expect("should have written to child rustfmt's stdin OK");

    let bindings = String::from_utf8(output)
        .expect("rustfmt should only emit valid utf-8");

    let stderr = stderr_handle.join()
        .expect("stderr reader thread should not have panicked")
        .expect("should have read child rustfmt's stderr OK");

    (bindings, stderr)
}

fn compare_generated_header(
    header: &PathBuf,
    builder: Builder,
) -> Result<(), Error> {
    let file_name = try!(header.file_name().ok_or(Error::new(
        ErrorKind::Other,
        "compare_generated_header expects a file",
    )));

    let mut expectation = PathBuf::from(header);
    expectation.pop();
    expectation.pop();
    expectation.push("expectations");
    expectation.push("tests");
    expectation.push(file_name);
    expectation.set_extension("rs");

    // If the expectation file doesn't exist, see if we have different test
    // expectations for different libclang versions.
    if !expectation.is_file() {
        let file_name = expectation.file_name().unwrap().to_owned();
        expectation.pop();

        if cfg!(feature = "testing_only_libclang_4") {
            expectation.push("libclang-4");
        } else if cfg!(feature = "testing_only_libclang_3_9") {
            expectation.push("libclang-3.9");
        } else if cfg!(feature = "testing_only_libclang_3_8") {
            expectation.push("libclang-3.8");
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
                    expectation.push(format!("libclang-{}", version_str));
                }
            }
        }

        expectation.push(file_name);

        if !expectation.is_file() {
            panic!(
                "missing test expectation file and/or 'testing_only_libclang_$VERSION' \
                    feature for header '{}'; looking for expectation file at '{}'",
                header.display(),
                expectation.display()
            );
        }
    }

    // We skip the generate() error here so we get a full diff below
    let (actual, rustfmt_stderr) = match builder.generate() {
        Ok(bindings) => {
            let actual = bindings.to_string();
            rustfmt(actual)
        }
        Err(()) => ("<error generating bindings>".to_string(), "".to_string()),
    };
    println!("{}", rustfmt_stderr);

    let mut expected = String::new();
    {
        if let Ok(expectation_file) = fs::File::open(&expectation) {
            try!(BufReader::new(expectation_file).read_to_string(&mut expected));
        }
    }

    let (expected, rustfmt_stderr) = rustfmt(expected);
    println!("{}", rustfmt_stderr);

    if actual == expected {
        if !actual.is_empty() {
            return Ok(());
        }
        return Err(Error::new(
            ErrorKind::Other,
            "Something's gone really wrong!",
        ));
    }

    println!("{}", rustfmt_stderr);

    println!("diff expected generated");
    println!("--- expected: {:?}", expectation);
    println!("+++ generated from: {:?}", header);

    for diff in diff::lines(&expected, &actual) {
        match diff {
            diff::Result::Left(l) => println!("-{}", l),
            diff::Result::Both(l, _) => println!(" {}", l),
            diff::Result::Right(r) => println!("+{}", r),
        }
    }

    // Override the diff.
    {
        let mut expectation_file = try!(fs::File::create(&expectation));
        try!(expectation_file.write_all(actual.as_bytes()));
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

    // Different platforms have various different conventions like struct padding, mangling, etc.
    // We make the default target as x86_64-unknown-linux
    if flags.iter().all(|flag| !flag.starts_with("--target=")) {
        if !flags.iter().any(|flag| flag == "--") {
            flags.push("--".into());
        }
        flags.push("--target=x86_64-unknown-linux".into());
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
        // We format in `compare_generated_header` ourselves to have a little
        // more control.
        "--no-rustfmt-bindings",
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
    let actual = builder()
        .header_contents("test.h", "int foo(const char* a);")
        .clang_arg("--target=x86_64-unknown-linux")
        .generate()
        .unwrap()
        .to_string();

    let (actual, stderr) = rustfmt(actual);
    println!("{}", stderr);

    let (expected, _) = rustfmt("/* automatically generated by rust-bindgen */

extern \"C\" {
    pub fn foo(a: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
".to_string());

    assert_eq!(
        expected,
        actual
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
        .clang_arg("--target=x86_64-unknown-linux")
        .generate()
        .unwrap()
        .to_string();

    let (actual, stderr) = rustfmt(actual);
    println!("{}", stderr);

    let expected = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/expectations/tests/test_multiple_header_calls_in_builder.rs"
    ));
    let (expected, _) = rustfmt(expected.to_string());

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
