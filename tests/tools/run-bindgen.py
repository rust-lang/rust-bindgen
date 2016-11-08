#!/usr/bin/env python

from __future__ import print_function

import argparse
import difflib
import os
import sys
import subprocess
import tempfile
import shlex

BINDGEN_FLAGS_PREFIX = "// bindgen-flags: "

COMMON_PRELUDE = """
#![allow(non_snake_case)]
"""

DESCRIPTION = """
Run bindgen on a test header and check the generated bindings against expected
output.
"""

def make_parser():
    """Make the commandline parser"""
    parser = argparse.ArgumentParser(description=DESCRIPTION)
    parser.add_argument("bindgen",
                        metavar="BINDGEN",
                        help="The path to the bindgen executable")
    parser.add_argument("header",
                        metavar="HEADER",
                        help="The path to the input header")
    parser.add_argument("rust_bindings",
                        metavar="RUST_BINDINGS",
                        help="The path to the generated rust output. If a file \
                              at this path already exists, the newly generated \
                              bindings will be checked against those extant \
                              expected bindings.")
    parser.add_argument("--feature",
                        dest="features",
                        action="append",
                        nargs=1,
                        help="Run tests that depend on bindgen being built with \
                              the given feature.")
    parser.add_argument("--dummy-uses",
                        dest="dummy_uses",
                        help="The path to generate dummy C/C++ uses of the \
                              whitelisted types from the input header at.")
    return parser

def usage_and_exit(*args):
    """Print the program usage and exit. If args are given, print them first"""
    if len(args) > 0:
        print(*args)
    make_parser().print_help()
    sys.exit(1)

def parse_args():
    """Get, parse, and validate commandline arguments."""
    parser = make_parser()
    args = parser.parse_args()

    if args.features is None:
        args.features = []

    if not os.path.isfile(args.bindgen):
        usage_and_exit("error: bindgen is not a file:", args.bindgen)

    if not os.path.isfile(args.header):
        usage_and_exit("error: header is not a file:", args.header)

    return args

def make_bindgen_env():
    """Build the environment to run bindgen in."""
    env = os.environ.copy()
    env["RUST_BACKTRACE"] = "1"

    # El Capitan likes to unset dyld variables
    # https://forums.developer.apple.com/thread/9233
    if "DYLD_LIBRARY_PATH" not in env and "LIBCLANG_PATH" in env:
        env["DYLD_LIBRARY_PATH"] = env["LIBCLANG_PATH"]

    return env

def get_bindgen_flags(header_path):
    """
    Return the bindgen flags required for this header
    """
    flags = ["--no-unstable-rust"]
    for line in COMMON_PRELUDE.split("\n"):
        flags.append("--raw-line")
        flags.append(line)

    with open(header_path) as f:
        for line in f:
            if line.startswith(BINDGEN_FLAGS_PREFIX):
                flags.extend(shlex.split(line.strip().split(BINDGEN_FLAGS_PREFIX)[1]))
                break

    return flags

def get_expected_bindings(rust_bindings_path):
    """
    Get the expected, generated rust bindings output, or None if there is no
    expected output yet.
    """
    expected_bindings = None
    if os.path.isfile(rust_bindings_path):
        with open(rust_bindings_path) as f:
            expected_bindings = f.read()
    return expected_bindings

def get_actual_bindings(rust_bindings_path):
    """Get the actual generated rust bindings output."""
    assert os.path.isfile(rust_bindings_path)
    with open(rust_bindings_path) as f:
        return f.read()

def run_cmd(command, **kwargs):
    """Run the given command, passing through **kwargs to subprocess.check_call"""
    print("run-bindgen.py: running", command)
    subprocess.check_call(command, **kwargs)

def generate_bindings(bindgen, dummy_uses, flags, header, output):
    """Generate the rust bindings."""
    command = [bindgen, "-o", output]
    if dummy_uses:
        command.extend(["--dummy-uses", dummy_uses])
    command.extend(flags)
    command.append(header)
    run_cmd(command, cwd=os.getcwd(), env=make_bindgen_env())

def check_actual_vs_expected(expected_bindings, rust_bindings_path):
    """
    Check the actual generated rust bindings versus our expected generated rust
    bindings. If they don't match up, print a diff between them and exit with a
    failure.
    """
    if expected_bindings is None:
        return

    actual_bindings = get_actual_bindings(rust_bindings_path)
    if actual_bindings == expected_bindings:
        return

    print("error: actual generated bindings do not match expected generated bindings!")

    def to_diffable(s):
        return map(lambda l: l + "\n", s.split("\n"))

    diff = difflib.unified_diff(to_diffable(expected_bindings),
                                to_diffable(actual_bindings),
                                fromfile="expected_bindings.rs",
                                tofile="actual_bindings.rs")
    sys.stderr.writelines(diff)
    sys.stderr.write("\n")

    sys.exit(1)

def main():
    args = parse_args()

    test_flags = get_bindgen_flags(args.header)
    expected_bindings = get_expected_bindings(args.rust_bindings)
    generate_bindings(args.bindgen,
                      args.dummy_uses,
                      test_flags,
                      args.header,
                      args.rust_bindings)
    check_actual_vs_expected(expected_bindings, args.rust_bindings)
    sys.exit(0)

if __name__ == "__main__":
    main()
