#!/usr/bin/env python3

import argparse
import os
import re
import shlex
import subprocess
import sys
from tempfile import NamedTemporaryFile

DESC = """

Determine whether `bindgen` can successfully process a C or C++ input header.

First, `bindgen` is run on the input header. Then the emitted bindings are
compiled with `rustc`. Finally, the compiled bindings' layout tests are run.

By default, this script will exit zero if all of the above steps are successful,
and non-zero if any of them fail. This is appropriate for determining if some
test case (perhaps generated with `csmith` or another fuzzer) uncovers any bugs
in `bindgen`.

However, this script can also be used when reducing (perhaps with `creduce`) a
known-bad test case into a new, smaller test case that exhibits the same bad
behavior. In this mode, you might expect that the emitted bindings fail to
compile with `rustc`, and want to exit non-zero early if that is not the
case. See the "reducing arguments" for details and what knobs are available.

"""

parser = argparse.ArgumentParser(
    formatter_class=argparse.RawDescriptionHelpFormatter,
    description=DESC.strip())

BINDGEN_ARGS = "--with-derive-partialeq \
--with-derive-eq \
--with-derive-partialord \
--with-derive-ord \
--with-derive-hash \
--with-derive-default"

parser.add_argument(
    "--bindgen-args",
    type=str,
    default=BINDGEN_ARGS,
    help="An argument string that `bindgen` should be invoked with. By default, all traits are derived. Note that the input header and output bindings file will automatically be provided by this script, and you should not manually specify them.")

parser.add_argument(
    "--save-temp-files",
    action="store_true",
    help="Do not delete temporary files.")

parser.add_argument(
    "input",
    type=str,
    default="input.h",
    help="The input header file. Defaults to 'input.h'.")

REDUCING_DESC = """

Arguments that are useful when reducing known-bad test cases into
equivalent-but-smaller test cases that exhibit the same bug with `creduce`.

"""

reducing = parser.add_argument_group("reducing arguments", REDUCING_DESC.strip())

reducing.add_argument(
    "--expect-bindgen-fail",
    action="store_true",
    help="Exit non-zero if `bindgen` successfully emits bindings.")
reducing.add_argument(
    "--bindgen-grep",
    type=str,
    help="Exit non-zero if the given regexp pattern is not found in `bindgen`'s output.")
reducing.add_argument(
    "--bindings-grep",
    type=str,
    help="Exit non-zero if the given regexp pattern is not found in the emitted bindings.")

reducing.add_argument(
    "--no-compile-bindings",
    action="store_false",
    dest="rustc",
    help="Do not attempt to compile the emitted bindings with `rustc`.")
reducing.add_argument(
    "--expect-compile-fail",
    action="store_true",
    help="Exit non-zero if `rustc` successfully compiles the emitted bindings.")
reducing.add_argument(
    "--rustc-grep",
    type=str,
    help="Exit non-zero if the output from compiling the bindings with `rustc` does not contain the given regexp pattern")

reducing.add_argument(
    "--no-layout-tests",
    action="store_false",
    dest="layout_tests",
    help="Do not run the compiled bindings' layout tests.")
reducing.add_argument(
    "--expect-layout-tests-fail",
    action="store_true",
    help="Exit non-zero if the compiled bindings' layout tests pass.")
reducing.add_argument(
    "--layout-tests-grep",
    type=str,
    help="Exit non-zero if the output of running the compiled bindings' layout tests does not contain the given regexp pattern.")

################################################################################

class ExitOne(Exception):
    pass

def exit_1(msg, child=None):
    print(msg)

    if child:
        stdout = decode(child.stdout)
        for line in stdout.splitlines():
            sys.stdout.write("+")
            sys.stdout.write(line)
            sys.stdout.write("\n")
        stderr = decode(child.stderr)
        for line in stderr.splitlines():
            sys.stderr.write("+")
            sys.stderr.write(line)
            sys.stderr.write("\n")

    raise ExitOne()

def main():
    args = parser.parse_args()
    os.environ["RUST_BACKTRACE"] = "full"

    exit_code = 0
    try:
        bindings = new_temp_file(prefix="bindings-", suffix=".rs")
        run_bindgen(args, bindings)

        if args.rustc and not args.expect_bindgen_fail:
            test_exe = new_temp_file(prefix="layout-tests-")
            run_rustc(args, bindings, test_exe)

            if args.layout_tests and not args.expect_compile_fail:
                run_layout_tests(args, test_exe)
    except ExitOne:
        exit_code = 1
    except Exception as e:
        exit_code = 2
        print("Unexpected exception:", e)

    if not args.save_temp_files:
        for path in TEMP_FILES:
            try:
                os.remove(path)
            except Exception as e:
                print("Unexpected exception:", e)

    sys.exit(exit_code)

def run(cmd, **kwargs):
    print("Running:", cmd)
    return subprocess.run(cmd, **kwargs)

def decode(f):
    return f.decode(encoding="utf-8", errors="ignore")

TEMP_FILES = []

def new_temp_file(prefix, suffix=None):
    temp = NamedTemporaryFile(delete=False, prefix=prefix, suffix=suffix)
    temp.close()
    TEMP_FILES.append(temp.name)
    return temp.name

def contains(pattern, lines):
    for line in lines:
        if re.match(pattern, line):
            return True
    return False

def regexp(pattern):
    if not pattern.startswith("^"):
        pattern = ".*" + pattern
    if not pattern.endswith("$"):
        pattern = pattern + ".*"
    return re.compile(pattern)

def run_bindgen(args, bindings):
    manifest_path = os.path.abspath(os.path.join(os.path.dirname(sys.argv[0]),
                                                 "..",
                                                 "Cargo.toml"))
    child = run(
        ["cargo", "run",
         "--manifest-path", manifest_path,
         "--",
         args.input, "-o", bindings] + shlex.split(args.bindgen_args),
        stderr=subprocess.PIPE,
        stdout=subprocess.PIPE)

    if args.bindgen_grep:
        pattern = regexp(args.bindgen_grep)
        if not (contains(pattern, decode(child.stdout).splitlines()) or
                contains(pattern, decode(child.stderr).splitlines())):
            exit_1("Error: did not find '{}' in `bindgen`'s output".format(args.bindgen_grep), child)

    if args.expect_bindgen_fail and child.returncode == 0:
        exit_1("Error: expected running `bindgen` to fail, but it didn't", child)

    if not args.expect_bindgen_fail and child.returncode != 0:
        exit_1("Error: running `bindgen` failed", child)

    if args.bindings_grep:
        pattern = regexp(args.bindings_grep)
        with open(bindings, mode="r") as f:
            if not contains(pattern, f):
                print("Error: expected the emitted bindings to contain '{}', but they didn't".format(args.bindings_grep))
                print("---------- {} ----------------------------------------------".format(bindings))
                f.seek(0)
                print(f.read())
                raise ExitOne()

def run_rustc(args, bindings, test_exe):
    child = run(
        ["rustc", "--crate-type", "lib", "--test", "-o", test_exe, bindings],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE)

    if args.rustc_grep:
        pattern = regexp(args.rustc_grep)
        if not (contains(pattern, decode(child.stdout).splitlines()) or
                contains(pattern, decode(child.stderr).splitlines())):
            exit_1("Error: did not find '{}' in `rustc`'s output".format(args.rustc_grep), child)

    if args.expect_compile_fail and child.returncode == 0:
        exit_1("Error: expected running `rustc` on the emitted bindings to fail, but it didn't", child)

    if not args.expect_compile_fail and child.returncode != 0:
        exit_1("Error: running `rustc` on the emitted bindings failed", child)

def run_layout_tests(args, test_exe):
    child = run(
        [test_exe],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE)

    if args.layout_tests_grep:
        pattern = regexp(args.layout_tests_grep)
        if not (contains(pattern, decode(child.stdout).splitlines()) or
                contains(pattern, decode(child.stderr).splitlines())):
            exit_1("Error: did not find '{}' in the compiled bindings' layout tests' output".format(args.layout_tests_grep), child)

    if args.expect_layout_tests_fail and child.returncode == 0:
        exit_1("Error: expected running the compiled bindings' layout tests to fail, but it didn't", child)

    if not args.expect_layout_tests_fail and child.returncode != 0:
        exit_1("Error: running the compiled bindings' layout tests failed", child)

if __name__ == "__main__":
    main()
