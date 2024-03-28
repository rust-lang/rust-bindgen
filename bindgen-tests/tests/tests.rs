use bindgen::{clang_version, Builder};
use owo_colors::{OwoColorize, Style};
use similar::{ChangeTag, TextDiff};
use std::env;
use std::fmt;
use std::fs;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};

use crate::options::builder_from_flags;

#[path = "../../bindgen-cli/options.rs"]
mod options;

mod parse_callbacks;

// Format the given source string. It can fail if the source string does not contain syntactically
// valid Rust.
fn format_code<S: AsRef<str>>(source: S) -> syn::Result<String> {
    use prettyplease::unparse;
    use syn::{parse_str, File};

    let file = parse_str::<File>(source.as_ref())?;
    Ok(unparse(&file))
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

    show_diff(expected, actual);

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

struct Line(Option<usize>);

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => write!(f, "    "),
            Some(idx) => write!(f, "{:<4}", idx + 1),
        }
    }
}

fn show_diff(old: &str, new: &str) {
    let diff = TextDiff::from_lines(old, new);
    for (count, group) in diff.grouped_ops(3).iter().enumerate() {
        if count > 0 {
            let message = format!("(chunk {count}/n)");
            println!("{}", message.cyan().dimmed());
        }
        for diff_op in group {
            for change in diff.iter_inline_changes(diff_op) {
                let (sign, color) = match change.tag() {
                    ChangeTag::Delete => ("-", Style::new().red()),
                    ChangeTag::Insert => ("+", Style::new().green()),
                    ChangeTag::Equal => (" ", Style::new()),
                };
                print!(
                    "{}{}| {}",
                    Line(change.old_index()).style(color).dimmed(),
                    Line(change.new_index()).style(color).dimmed(),
                    sign.style(color).bold(),
                );
                for (emphasized, text) in change.iter_strings_lossy() {
                    if emphasized {
                        print!("{}", text.style(color).underline());
                    } else {
                        print!("{}", text.style(color));
                    }
                }
                if change.missing_newline() {
                    println!();
                }
            }
        }
    }
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

        if cfg!(feature = "__testing_only_libclang_16") {
            expectation.push("libclang-16");
        } else if cfg!(feature = "__testing_only_libclang_9") {
            expectation.push("libclang-9");
        } else {
            match clang_version().parsed {
                None => expectation.push("libclang-16"),
                Some(version) => {
                    let (maj, min) = version;
                    let version_str = if maj >= 16 {
                        "16".to_owned()
                    } else if maj >= 9 {
                        "9".to_owned()
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
            "missing test expectation file and/or '__testing_only_libclang_$VERSION' \
             feature for header '{}'; looking for expectation file at '{:?}'",
            header.display(),
            looked_at,
        ),
    };

    let (builder, roundtrip_builder) = builder.into_builder(check_roundtrip)?;

    // We skip the generate() error here so we get a full diff below
    let actual = match builder.generate() {
        Ok(bindings) => format_code(bindings.to_string()).map_err(|err| {
            Error::new(
                ErrorKind::Other,
                format!("Cannot parse the generated bindings: {}", err),
            )
        })?,
        Err(_) => "/* error generating bindings */\n".into(),
    };

    if actual.is_empty() {
        return Err(Error::new(
            ErrorKind::Other,
            "Something's gone really wrong!",
        ));
    }
    if actual != expected {
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
            flags.extend(extra_flags);
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
        "--formatter=none",
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

    let args = prepend.iter().map(ToString::to_string).chain(flags);

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
             #if defined NOT_THREE && NOT_THREE == 1\nextern const int z[] = { 42 };\n#endif\n",
        )
        .generate()
        .unwrap()
        .to_string();

    let actual = format_code(actual).unwrap();

    let expected = format_code(
        "extern \"C\" {
    pub static x: [::std::os::raw::c_int; 1usize];
}
extern \"C\" {
    pub static y: [::std::os::raw::c_int; 1usize];
}
",
    )
    .unwrap();

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

    let actual = format_code(actual).unwrap();

    let expected = format_code(
        "extern \"C\" {
    pub fn foo(a: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
",
    )
    .unwrap();

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

    let actual = format_code(actual).unwrap();

    let expected_filename = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/expectations/tests/test_multiple_header_calls_in_builder.rs"
    );
    let expected = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/expectations/tests/test_multiple_header_calls_in_builder.rs"
    ));
    let expected = format_code(expected).unwrap();

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
fn test_headers_call_in_builder() {
    let actual = builder()
        .headers([
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/headers/func_ptr.h"),
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/headers/char.h"),
        ])
        .clang_arg("--target=x86_64-unknown-linux")
        .generate()
        .unwrap()
        .to_string();

    let actual = format_code(actual).unwrap();

    let expected_filename = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/expectations/tests/test_multiple_header_calls_in_builder.rs"
    );
    let expected = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/expectations/tests/test_multiple_header_calls_in_builder.rs"
    ));
    let expected = format_code(expected).unwrap();

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

    let actual = format_code(actual).unwrap();

    let expected = format_code(
        "extern \"C\" {
    pub fn foo2(b: *const ::std::os::raw::c_char) -> f32;
}
extern \"C\" {
    pub fn foo(a: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
",
    )
    .unwrap();

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

    let actual = format_code(actual).unwrap();

    let expected_filename = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/expectations/tests/test_mixed_header_and_header_contents.rs"
    );
    let expected = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/expectations/tests/test_mixed_header_and_header_contents.rs"
    ));
    let expected = format_code(expected).unwrap();
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

fn build_flags_output_helper(builder: &bindgen::Builder) {
    let mut command_line_flags = builder.command_line_flags();
    command_line_flags.insert(0, "bindgen".to_string());

    let flags_quoted: Vec<String> = command_line_flags
        .iter()
        .map(|x| format!("{}", shlex::try_quote(x).unwrap()))
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

#[test]
fn test_wrap_static_fns() {
    // This test is for testing diffs of the generated C source and header files
    // TODO: If another such feature is added, convert this test into a more generic
    //      test that looks at `tests/headers/generated` directory.
    let expect_path = PathBuf::from("tests/expectations/tests/generated")
        .join("wrap_static_fns");
    println!("In path is ::: {}", expect_path.display());

    let generated_path =
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("wrap_static_fns");
    println!("Out path is ::: {}", generated_path.display());

    let _bindings = Builder::default()
        .header("tests/headers/wrap-static-fns.h")
        .wrap_static_fns(true)
        .wrap_static_fns_path(generated_path.display().to_string())
        .parse_callbacks(Box::new(parse_callbacks::WrapAsVariadicFn))
        .generate()
        .expect("Failed to generate bindings");

    let expected_c = fs::read_to_string(expect_path.with_extension("c"))
        .expect("Could not read generated wrap_static_fns.c");

    let actual_c = fs::read_to_string(generated_path.with_extension("c"))
        .expect("Could not read actual wrap_static_fns.c");

    if expected_c != actual_c {
        error_diff_mismatch(
            &actual_c,
            &expected_c,
            None,
            &expect_path.with_extension("c"),
        )
        .unwrap();
    }
}
