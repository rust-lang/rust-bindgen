extern crate bindgen;
extern crate clap;
extern crate diff;
#[cfg(feature = "logging")]
extern crate env_logger;
extern crate shlex;

use bindgen::{clang_version, Builder};
use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader, Error, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::sync::Once;

use crate::options::builder_from_flags;

#[path = "../../bindgen-cli/options.rs"]
mod options;

mod parse_callbacks;

// Run `rustfmt` on the given source string and return a tuple of the formatted
// bindings, and rustfmt's stderr.
fn rustfmt(source: String) -> (String, String) {
    static CHECK_RUSTFMT: Once = Once::new();

    CHECK_RUSTFMT.call_once(|| {
        if env::var_os("RUSTFMT").is_some() {
            return;
        }

        let mut rustfmt = {
            let mut p = process::Command::new("rustup");
            p.args(["run", "nightly", "rustfmt", "--version"]);
            p
        };

        let have_working_rustfmt = rustfmt
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .status()
            .ok()
            .map_or(false, |status| status.success());

        if !have_working_rustfmt {
            panic!(
                "
The latest `rustfmt` is required to run the `bindgen` test suite. Install
`rustfmt` with:

    $ rustup update nightly
    $ rustup component add rustfmt --toolchain nightly
"
            );
        }
    });

    let mut child = match env::var_os("RUSTFMT") {
        Some(r) => process::Command::new(r),
        None => {
            let mut p = process::Command::new("rustup");
            p.args(["run", "nightly", "rustfmt"]);
            p
        }
    };

    let mut child = child
        .args([
            "--config-path",
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/rustfmt.toml"),
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
    let stdin_handle =
        ::std::thread::spawn(move || stdin.write_all(source.as_bytes()));

    // Read stderr on a new thread for similar reasons.
    let stderr_handle = ::std::thread::spawn(move || {
        let mut output = vec![];
        io::copy(&mut stderr, &mut output)
            .map(|_| String::from_utf8_lossy(&output).to_string())
    });

    let mut output = vec![];
    io::copy(&mut stdout, &mut output).expect("Should copy stdout into vec OK");

    // Ignore actual rustfmt status because it is often non-zero for trivial
    // things.
    let _ = child.wait().expect("should wait on rustfmt child OK");

    stdin_handle
        .join()
        .expect("writer thread should not have panicked")
        .expect("should have written to child rustfmt's stdin OK");

    let bindings = String::from_utf8(output)
        .expect("rustfmt should only emit valid utf-8");

    let stderr = stderr_handle
        .join()
        .expect("stderr reader thread should not have panicked")
        .expect("should have read child rustfmt's stderr OK");

    (bindings, stderr)
}

fn should_overwrite_expected() -> bool {
    if let Some(var) = env::var_os("BINDGEN_OVERWRITE_EXPECTED") {
        if var == "1" {
            return true;
        }
        if var != "0" && var != "" {
            panic!("Invalid value of BINDGEN_OVERWRITE_EXPECTED");
        }
    }
    false
}

fn error_diff_mismatch(
    actual: &str,
    expected: &str,
    header: Option<&Path>,
    filename: &Path,
) -> Result<(), Error> {
    println!("diff expected generated");
    println!("--- expected: {:?}", filename);
    if let Some(header) = header {
        println!("+++ generated from: {:?}", header);
    }

    for diff in diff::lines(expected, actual) {
        match diff {
            diff::Result::Left(l) => println!("-{}", l),
            diff::Result::Both(l, _) => println!(" {}", l),
            diff::Result::Right(r) => println!("+{}", r),
        }
    }

    if should_overwrite_expected() {
        // Overwrite the expectation with actual output.
        let mut expectation_file = fs::File::create(filename)?;
        expectation_file.write_all(actual.as_bytes())?;
    }

    if let Some(var) = env::var_os("BINDGEN_TESTS_DIFFTOOL") {
        //usecase: var = "meld" -> You can hand check differences
        let name = match filename.components().last() {
            Some(std::path::Component::Normal(name)) => name,
            _ => panic!("Why is the header variable so weird?"),
        };
        let actual_result_path =
            PathBuf::from(env::var("OUT_DIR").unwrap()).join(name);
        let mut actual_result_file = fs::File::create(&actual_result_path)?;
        actual_result_file.write_all(actual.as_bytes())?;
        std::process::Command::new(var)
            .args([filename, &actual_result_path])
            .output()?;
    }

    Err(Error::new(ErrorKind::Other, "Header and binding differ! Run with BINDGEN_OVERWRITE_EXPECTED=1 in the environment to automatically overwrite the expectation or with BINDGEN_TESTS_DIFFTOOL=meld to do this manually."))
}

fn compare_generated_header(
    header: &Path,
    builder: BuilderState,
    check_roundtrip: bool,
) -> Result<(), Error> {
    let file_name = header.file_name().ok_or_else(|| {
        Error::new(ErrorKind::Other, "compare_generated_header expects a file")
    })?;

    let mut expectation = PathBuf::from(header);
    expectation.pop();
    expectation.pop();
    expectation.push("expectations");
    expectation.push("tests");

    let mut looked_at = vec![];
    let mut expectation_file;

    // Try more specific expectations first.
    {
        let mut expectation = expectation.clone();

        if cfg!(feature = "testing_only_libclang_9") {
            expectation.push("libclang-9");
        } else if cfg!(feature = "testing_only_libclang_5") {
            expectation.push("libclang-5");
        } else {
            match clang_version().parsed {
                None => expectation.push("libclang-9"),
                Some(version) => {
                    let (maj, min) = version;
                    let version_str = if maj >= 9 {
                        "9".to_owned()
                    } else if maj >= 5 {
                        "5".to_owned()
                    } else if maj >= 4 {
                        "4".to_owned()
                    } else {
                        format!("{}.{}", maj, min)
                    };
                    expectation.push(format!("libclang-{}", version_str));
                }
            }
        }

        expectation.push(file_name);
        expectation.set_extension("rs");
        expectation_file = fs::File::open(&expectation).ok();
        looked_at.push(expectation);
    }

    // Try the default path otherwise.
    if expectation_file.is_none() {
        expectation.push(file_name);
        expectation.set_extension("rs");
        expectation_file = fs::File::open(&expectation).ok();
        looked_at.push(expectation.clone());
    }

    let mut expected = String::new();
    match expectation_file {
        Some(f) => {
            BufReader::new(f).read_to_string(&mut expected)?;
        }
        None => panic!(
            "missing test expectation file and/or 'testing_only_libclang_$VERSION' \
             feature for header '{}'; looking for expectation file at '{:?}'",
            header.display(),
            looked_at,
        ),
    };

    let (builder, roundtrip_builder) = builder.into_builder(check_roundtrip)?;

    // We skip the generate() error here so we get a full diff below
    let (actual, rustfmt_stderr) = match builder.generate() {
        Ok(bindings) => {
            let actual = bindings.to_string();
            rustfmt(actual)
        }
        Err(_) => ("/* error generating bindings */\n".into(), "".to_string()),
    };
    println!("{}", rustfmt_stderr);

    let (expected, rustfmt_stderr) = rustfmt(expected);
    println!("{}", rustfmt_stderr);

    if actual.is_empty() {
        return Err(Error::new(
            ErrorKind::Other,
            "Something's gone really wrong!",
        ));
    }

    if actual != expected {
        println!("{}", rustfmt_stderr);
        return error_diff_mismatch(
            &actual,
            &expected,
            Some(header),
            looked_at.last().unwrap(),
        );
    }

    if let Some(roundtrip_builder) = roundtrip_builder {
        if let Err(e) =
            compare_generated_header(header, roundtrip_builder, false)
        {
            return Err(Error::new(ErrorKind::Other, format!("Checking CLI flags roundtrip errored! You probably need to fix Builder::command_line_flags. {}", e)));
        }
    }

    Ok(())
}

fn builder() -> Builder {
    #[cfg(feature = "logging")]
    let _ = env_logger::try_init();

    bindgen::builder().disable_header_comment()
}

struct BuilderState {
    builder: Builder,
    parse_callbacks: Option<String>,
}

impl BuilderState {
    fn into_builder(
        self,
        with_roundtrip_builder: bool,
    ) -> Result<(Builder, Option<BuilderState>), Error> {
        let roundtrip_builder = if with_roundtrip_builder {
            let mut flags = self.builder.command_line_flags();
            flags.insert(0, "bindgen".into());
            let mut builder = builder_from_flags(flags.into_iter())?.0;
            if let Some(ref parse_cb) = self.parse_callbacks {
                builder =
                    builder.parse_callbacks(parse_callbacks::lookup(parse_cb));
            }
            Some(BuilderState {
                builder,
                parse_callbacks: self.parse_callbacks,
            })
        } else {
            None
        };
        Ok((self.builder, roundtrip_builder))
    }
}

fn create_bindgen_builder(header: &Path) -> Result<BuilderState, Error> {
    #[cfg(feature = "logging")]
    let _ = env_logger::try_init();

    let source = fs::File::open(header)?;
    let reader = BufReader::new(source);

    // Scoop up bindgen-flags from test header
    let mut flags = Vec::with_capacity(2);
    let mut parse_callbacks = None;

    for line in reader.lines() {
        let line = line?;
        if !line.starts_with("// bindgen") {
            continue;
        }

        if line.contains("bindgen-flags: ") {
            let extra_flags = line
                .split("bindgen-flags: ")
                .last()
                .and_then(shlex::split)
                .unwrap();
            flags.extend(extra_flags.into_iter());
        } else if line.contains("bindgen-osx-only") {
            let prepend_flags = ["--raw-line", "#![cfg(target_os=\"macos\")]"];
            flags = prepend_flags
                .iter()
                .map(ToString::to_string)
                .chain(flags)
                .collect();
        } else if line.contains("bindgen-parse-callbacks: ") {
            let parse_cb =
                line.split("bindgen-parse-callbacks: ").last().unwrap();
            parse_callbacks = Some(parse_cb.to_owned());
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
    let header_str = header.to_str().ok_or_else(|| {
        Error::new(ErrorKind::Other, "Invalid header file name")
    })?;

    let prepend = [
        "bindgen",
        // We format in `compare_generated_header` ourselves to have a little
        // more control.
        "--no-rustfmt-bindings",
        "--with-derive-default",
        "--disable-header-comment",
        "--vtable-generation",
        header_str,
        "--raw-line",
        "",
        "--raw-line",
        "#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]",
        "--raw-line",
        "",
    ];

    let args = prepend
        .iter()
        .map(ToString::to_string)
        .chain(flags.into_iter());

    let mut builder = builder_from_flags(args)?.0;
    if let Some(ref parse_cb) = parse_callbacks {
        builder = builder.parse_callbacks(parse_callbacks::lookup(parse_cb));
    }
    Ok(BuilderState {
        builder,
        parse_callbacks,
    })
}

macro_rules! test_header {
    ($function:ident, $header:expr) => {
        #[test]
        fn $function() {
            let header = PathBuf::from($header);
            let result = create_bindgen_builder(&header).and_then(|builder| {
                let check_roundtrip =
                    env::var_os("BINDGEN_DISABLE_ROUNDTRIP_TEST").is_none();
                compare_generated_header(&header, builder, check_roundtrip)
            });

            if let Err(err) = result {
                panic!("{}", err);
            }
        }
    };
}

// This file is generated by build.rs
include!(concat!(env!("OUT_DIR"), "/tests.rs"));

#[test]
#[cfg_attr(target_os = "windows", ignore)]
fn test_clang_env_args() {
    std::env::set_var(
        "BINDGEN_EXTRA_CLANG_ARGS",
        "-D_ENV_ONE=1 -D_ENV_TWO=\"2 -DNOT_THREE=1\"",
    );
    let actual = builder()
        .disable_header_comment()
        .header_contents(
            "test.hpp",
            "#ifdef _ENV_ONE\nextern const int x[] = { 42 };\n#endif\n\
             #ifdef _ENV_TWO\nextern const int y[] = { 42 };\n#endif\n\
             #ifdef NOT_THREE\nextern const int z[] = { 42 };\n#endif\n",
        )
        .generate()
        .unwrap()
        .to_string();

    let (actual, stderr) = rustfmt(actual);
    println!("{}", stderr);

    let (expected, _) = rustfmt(
        "extern \"C\" {
    pub static x: [::std::os::raw::c_int; 1usize];
}
extern \"C\" {
    pub static y: [::std::os::raw::c_int; 1usize];
}
"
        .to_string(),
    );

    assert_eq!(expected, actual);
}

#[test]
fn test_header_contents() {
    let actual = builder()
        .disable_header_comment()
        .header_contents("test.h", "int foo(const char* a);")
        .clang_arg("--target=x86_64-unknown-linux")
        .generate()
        .unwrap()
        .to_string();

    let (actual, stderr) = rustfmt(actual);
    println!("{}", stderr);

    let (expected, _) = rustfmt(
        "extern \"C\" {
    pub fn foo(a: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
"
        .to_string(),
    );

    assert_eq!(expected, actual);
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

    let expected_filename = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/expectations/tests/test_multiple_header_calls_in_builder.rs"
    );
    let expected = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/expectations/tests/test_multiple_header_calls_in_builder.rs"
    ));
    let (expected, _) = rustfmt(expected.to_string());

    if actual != expected {
        println!("Generated bindings differ from expected!");
        error_diff_mismatch(
            &actual,
            &expected,
            None,
            Path::new(expected_filename),
        )
        .unwrap();
    }
}

#[test]
fn test_multiple_header_contents() {
    let actual = builder()
        .header_contents("test.h", "int foo(const char* a);")
        .header_contents("test2.h", "float foo2(const char* b);")
        .clang_arg("--target=x86_64-unknown-linux")
        .generate()
        .unwrap()
        .to_string();

    let (actual, stderr) = rustfmt(actual);
    println!("{}", stderr);

    let (expected, _) = rustfmt(
        "extern \"C\" {
    pub fn foo2(b: *const ::std::os::raw::c_char) -> f32;
}
extern \"C\" {
    pub fn foo(a: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
"
        .to_string(),
    );

    assert_eq!(expected, actual);
}

#[test]
fn test_mixed_header_and_header_contents() {
    let actual = builder()
        .header(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/headers/func_ptr.h"
        ))
        .header(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/headers/char.h"))
        .header_contents("test.h", "int bar(const char* a);")
        .header_contents("test2.h", "float bar2(const char* b);")
        .clang_arg("--target=x86_64-unknown-linux")
        .generate()
        .unwrap()
        .to_string();

    let (actual, stderr) = rustfmt(actual);
    println!("{}", stderr);

    let expected_filename = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/expectations/tests/test_mixed_header_and_header_contents.rs"
    );
    let expected = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/expectations/tests/test_mixed_header_and_header_contents.rs"
    ));
    let (expected, _) = rustfmt(expected.to_string());
    if expected != actual {
        error_diff_mismatch(
            &actual,
            &expected,
            None,
            Path::new(expected_filename),
        )
        .unwrap();
    }
}

#[test]
// Doesn't support executing sh file on Windows.
// We may want to implement it in Rust so that we support all systems.
#[cfg(not(target_os = "windows"))]
fn no_system_header_includes() {
    use std::process::Command;
    assert!(Command::new("../ci/no-includes.sh")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .spawn()
        .expect("should spawn ../ci/no-includes.sh OK")
        .wait()
        .expect("should wait for ../ci/no-includes OK")
        .success());
}

#[test]
fn emit_depfile() {
    let header = PathBuf::from("tests/headers/enum-default-rust.h");
    let expected_depfile = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("expectations")
        .join("tests")
        .join("enum-default-rust.d");
    let observed_depfile = tempfile::NamedTempFile::new().unwrap();
    let mut builder = create_bindgen_builder(&header).unwrap();
    builder.builder = builder.builder.depfile(
        "tests/expectations/tests/enum-default-rust.rs",
        observed_depfile.path(),
    );

    let check_roundtrip =
        env::var_os("BINDGEN_DISABLE_ROUNDTRIP_TEST").is_none();
    let (builder, _roundtrip_builder) =
        builder.into_builder(check_roundtrip).unwrap();
    let _bindings = builder.generate().unwrap();

    let observed = std::fs::read_to_string(observed_depfile).unwrap();
    let expected = std::fs::read_to_string(expected_depfile).unwrap();
    assert_eq!(observed.trim(), expected.trim());
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
        bindgen_ii.contains(&arg_keyword),
        "arg_keyword.hpp is in the preprocessed file"
    );
    assert!(
        bindgen_ii.contains(&empty_layout),
        "cpp-empty-layout.hpp is in the preprocessed file"
    );
}

#[test]
fn allowlist_warnings() {
    let header = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/headers/allowlist_warnings.h"
    );

    let bindings = builder()
        .header(header)
        .allowlist_function("doesnt_match_anything")
        .generate()
        .expect("unable to generate bindings");

    assert_eq!(1, bindings.warnings().len());
}

fn build_flags_output_helper(builder: &bindgen::Builder) {
    let mut command_line_flags = builder.command_line_flags();
    command_line_flags.insert(0, "bindgen".to_string());

    let flags_quoted: Vec<String> = command_line_flags
        .iter()
        .map(|x| format!("{}", shlex::quote(x)))
        .collect();
    let flags_str = flags_quoted.join(" ");
    println!("{}", flags_str);

    let (builder, _output, _verbose) =
        crate::options::builder_from_flags(command_line_flags.into_iter())
            .unwrap();
    builder.generate().expect("failed to generate bindings");
}

#[test]
fn commandline_multiple_headers() {
    let bindings = bindgen::Builder::default()
        .header("tests/headers/char.h")
        .header("tests/headers/func_ptr.h")
        .header("tests/headers/16-byte-alignment.h");
    build_flags_output_helper(&bindings);
}
