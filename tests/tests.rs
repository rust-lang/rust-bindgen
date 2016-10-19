// We add this `extern crate` here to ensure that bindgen is up-to-date and
// rebuilt, even though we aren't using any of its types or functions here, only
// indirectly calling the executable.
#[allow(dead_code)]
extern crate bindgen;

use std::env;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process;

fn spawn_run_bindgen<P, Q, R>(run_bindgen: P, bindgen: Q, header: R) -> process::Child
    where P: AsRef<Path>,
          Q: AsRef<Path>,
          R: AsRef<Path>
{
    let run_bindgen = run_bindgen.as_ref();
    let bindgen = bindgen.as_ref();
    let header = header.as_ref();

    // Convert from "tests/headers/foo.hpp" to "tests/expectations/foo.rs" by
    // saving the filename, popping off "headers/foo.hpp", pushing
    // "expectations", pushing the saved filename, and finally modifying the
    // extension.

    let mut expected = PathBuf::from(header);
    let file_name = expected.file_name()
        .expect("Should have filename")
        .to_os_string();
    expected.pop();
    expected.pop();
    expected.push("expectations");
    expected.push(file_name);
    expected.set_extension("rs");

    let mut cmd = process::Command::new(run_bindgen);
    cmd.stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .arg(bindgen)
        .arg(header)
        .arg(expected);

    if cfg!(feature = "llvm_stable") {
        cmd.arg("--feature")
            .arg("llvm_stable");
    }

    cmd.spawn()
        .expect("Should be able to spawn run-bindgen.py child process")
}

#[test]
fn run_bindgen_tests() {
    let crate_root = env::var("CARGO_MANIFEST_DIR")
        .expect("should have CARGO_MANIFEST_DIR environment variable");

    let mut run_bindgen = PathBuf::from(&crate_root);
    run_bindgen.push("tests");
    run_bindgen.push("tools");
    run_bindgen.push("run-bindgen.py");

    let mut bindgen = PathBuf::from(&crate_root);
    bindgen.push("target");
    bindgen.push("debug");
    bindgen.push("bindgen");
    if !bindgen.is_file() {
        panic!("{} is not a file! Build bindgen before running tests.",
               bindgen.display());
    }

    let mut headers_dir = PathBuf::from(&crate_root);
    headers_dir.push("tests");
    headers_dir.push("headers");

    let entries = fs::read_dir(&headers_dir)
        .expect("Should read directory")
        .map(|result| result.expect("Should read directory entry"));

    let tests = entries.filter(|entry| {
        match entry.path().extension().map(|s| s.to_str()) {
            Some(Some("h")) |
            Some(Some("hpp")) => true,
            _ => false,
        }
    });

    // First spawn all child processes and collect them, then wait on each
    // one. This runs the tests in parallel rather than serially.

    let children: Vec<_> = tests.map(|entry| {
            let child = spawn_run_bindgen(run_bindgen.clone(), bindgen.clone(), entry.path());
            (entry.path(), child)
        })
        .collect();

    let failures: Vec<_> = children.into_iter()
        .filter_map(|(path, mut child)| {
            let passed = child.wait()
                .expect("Should wait on child process")
                .success();

            if passed { None } else { Some((path, child)) }
        })
        .collect();

    let num_failures = failures.len();

    for (path, child) in failures {
        println!("FAIL: {}", path.display());

        let mut buf = String::new();

        child.stdout
            .expect("should have stdout piped")
            .read_to_string(&mut buf)
            .expect("should read child's stdout");
        for line in buf.lines() {
            println!("child stdout> {}", line);
        }

        child.stderr
            .expect("should have stderr piped")
            .read_to_string(&mut buf)
            .expect("should read child's stderr");
        for line in buf.lines() {
            println!("child stderr> {}", line);
        }
    }

    if num_failures > 0 {
        panic!("{} test failures!", num_failures);
    }
}
