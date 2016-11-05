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

const TEST_BATCH_DEFAULT_SIZE: usize = 16;

fn spawn_run_bindgen<P, Q, R>(run_bindgen: P,
                              bindgen: Q,
                              header: R)
                              -> process::Child
    where P: AsRef<Path>,
          Q: AsRef<Path>,
          R: AsRef<Path>,
{
    let run_bindgen = run_bindgen.as_ref();
    let bindgen = bindgen.as_ref();
    let header = header.as_ref();

    // Convert from "tests/headers/foo.hpp" to "tests/expectations/tests/foo.rs" by
    // saving the filename, popping off "headers/foo.hpp", pushing
    // "expectations/tests", pushing the saved filename, and finally modifying the
    // extension.

    let mut expected = PathBuf::from(header);
    let file_name = expected.file_name()
        .expect("Should have filename")
        .to_os_string();
    expected.pop();
    expected.pop();
    expected.push("expectations");
    expected.push("tests");
    expected.push(file_name);
    expected.set_extension("rs");

    // And the same style conversion as above, but for the dummy uses. We assume
    // that .hpp means we should generate a .cpp uses file, and .h means we
    // should generate a .c file.

    let mut dummy_uses = PathBuf::from(header);
    let file_name = dummy_uses.file_name()
        .expect("Should still have filename")
        .to_os_string();
    dummy_uses.pop();
    dummy_uses.pop();
    dummy_uses.push("uses");
    dummy_uses.push(file_name);
    dummy_uses.set_extension(if header.extension().and_then(|s| s.to_str()) ==
                                Some("hpp") {
        "cpp"
    } else {
        "c"
    });

    process::Command::new(run_bindgen)
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .arg(bindgen)
        .arg(header)
        .arg(expected)
        .arg("--dummy-uses")
        .arg(dummy_uses)
        .spawn()
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
    if cfg!(debug_assertions) {
        bindgen.push("debug");
    } else {
        bindgen.push("release");
    }
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
            match entry.path().extension().and_then(|s| s.to_str()) {
                Some("h") | Some("hpp") => true,
                _ => false,
            }
        })
        .collect::<Vec<_>>();

    let batch_size = env::var("BINDGEN_TEST_BATCH_SIZE")
        .ok()
        .and_then(|x| x.parse::<usize>().ok())
        .unwrap_or(TEST_BATCH_DEFAULT_SIZE);

    // Spawn `batch_size` children to run in parallel and wait on all of them
    // before processing the next batch. This puts a limit on the resources
    // consumed when testing, so that we don't overload the system.

    let children = tests.chunks(batch_size).map(|x| {
        x.iter()
            .map(|entry| {
                let child = spawn_run_bindgen(run_bindgen.clone(),
                                              bindgen.clone(),
                                              entry.path());
                (entry.path(), child)
            })
            .collect::<Vec<_>>()
    });

    let failures: Vec<_> = children.flat_map(|x| {
            x.into_iter().filter_map(|(path, mut child)| {
                let passed = child.wait()
                    .expect("Should wait on child process")
                    .success();

                if passed { None } else { Some((path, child)) }
            })
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
