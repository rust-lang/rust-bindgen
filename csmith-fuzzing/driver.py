#!/usr/bin/env python3

import argparse
import os
import re
import shlex
import sys
from subprocess import run, SubprocessError, DEVNULL, PIPE
from tempfile import NamedTemporaryFile

DESC = """

A `csmith` fuzzing driver for `bindgen`.

Generates random C source files with `csmith` and then passes them to `bindgen`
(via `predicate.py`). If `bindgen` can't emit bindings, `rustc` can't compile
those bindings, or the compiled bindings' layout tests fail, then the driver has
found a bug, and will report the problematic test case to you.

"""

parser = argparse.ArgumentParser(
    formatter_class=argparse.RawDescriptionHelpFormatter,
    description=DESC.strip())

parser.add_argument(
    "--keep-going",
    action="store_true",
    help="Do not stop after finding a test case that exhibits a bug in `bindgen`. Instead, keep going.")

CSMITH_ARGS="\
--no-checksum \
--nomain \
--max-block-size 1 \
--max-block-depth 1"

parser.add_argument(
    "--csmith-args",
    type=str,
    default=CSMITH_ARGS,
    help="Pass this argument string to `csmith`. By default, very small functions are generated.")

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
    help="Pass this argument string to `bindgen`. By default, all traits are derived.")

parser.add_argument(
    "--no-creduce",
    action="store_false",
    dest="creduce",
    help="Do not run `creduce` on any buggy test case(s) discovered.")

################################################################################

def cat(path, title=None):
    if not title:
        title = path
    print("-------------------- {} --------------------".format(title))
    print()
    print()
    run(["cat", path])

def decode(f):
    return f.decode(encoding="utf-8", errors="ignore")

def run_logged(cmd):
    result = run(cmd, stdin=DEVNULL, stdout=PIPE, stderr=PIPE)
    result.stdout = decode(result.stdout)
    result.stderr = decode(result.stderr)
    if result.returncode != 0:
        print()
        print()
        print("Error: {} exited with code {}".format(cmd, result.returncode))
        print()
        print()
        for line in result.stdout.splitlines():
            sys.stdout.write("+")
            sys.stdout.write(line)
            sys.stdout.write("\n")
        for line in result.stderr.splitlines():
            sys.stderr.write("+")
            sys.stderr.write(line)
            sys.stderr.write("\n")
    return result

def main():
    os.environ["RUST_BACKTRACE"] = "full"
    args = parser.parse_args()

    bindgen_args = args.bindgen_args
    if bindgen_args.find(" -- ") == -1:
        bindgen_args = bindgen_args + " -- "
    bindgen_args = bindgen_args + " -I{}".format(os.path.abspath(os.path.dirname(sys.argv[0])))
    args.bindgen_args = bindgen_args

    print()
    print()
    print("Fuzzing `bindgen` with C-Smith...")
    print()
    print()

    iterations = 0
    while True:
        print("\rIteration: {}".format(iterations), end="", flush=True)
        iterations += 1

        input = NamedTemporaryFile(delete=False, prefix="input-", suffix=".h")
        input.close()
        result = run_logged(["csmith", "-o", input.name] + shlex.split(args.csmith_args))
        if result.returncode != 0:
            exit(1)

        predicate_command = [
            "./predicate.py",
            "--bindgen-args",
            args.bindgen_args,
            input.name
        ]
        result = run_logged(predicate_command)

        if result.returncode != 0:
            print()
            print()
            cat(input.name, title="Failing test case: {}".format(input.name))
            print()
            print()

            if args.creduce:
                creduce(args, input.name, result)

            print_issue_template(args, input.name, predicate_command, result)

            if args.keep_going:
                continue
            exit(1)

        os.remove(input.name)

RUSTC_ERROR_REGEX = re.compile(r".*(error\[.*].*)")
LAYOUT_TEST_FAILURE = re.compile(r".*(test bindgen_test_layout_.* \.\.\. FAILED)")

def creduce(args, failing_test_case, result):
    print()
    print()
    print("Reducing failing test case with `creduce`...")

    match = re.search(RUSTC_ERROR_REGEX, result.stderr)
    if match:
        error_msg = match.group(1)
        print("...searching for \"{}\".".format(error_msg))
        return creduce_with_predicate_flags(
            args,
            failing_test_case,
            "--bindgen-args '{}' --expect-compile-fail --rustc-grep '{}'".format(
                args.bindgen_args,
                re.escape(error_msg)
            )
        )

    match = re.search(LAYOUT_TEST_FAILURE, result.stdout)
    if match:
        layout_failure = match.group(1)
        struct_name = layout_failure[len("test bindgen_test_layout_"):layout_failure.rindex(" ... FAILED")]
        print("...searching for \"{}\".".format(layout_failure))
        return creduce_with_predicate_flags(
            args,
            failing_test_case,
            "--bindgen-args '{}' --expect-layout-tests-fail --bindings-grep '{}' --layout-tests-grep '{}'".format(
                args.bindgen_args,
                re.escape(struct_name),
                re.escape(layout_failure)
            )
        )

    print("...nevermind, don't know how to `creduce` this bug. Skipping.")

def creduce_with_predicate_flags(args, failing_test_case, predicate_flags):
    predicate = """
#!/usr/bin/env bash
set -eu
{} {} {}
    """.format(
        os.path.abspath(os.path.join(os.path.dirname(sys.argv[0]), "predicate.py")),
        predicate_flags,
        os.path.basename(failing_test_case)
    )

    print("...and reducing with this script:")
    print()
    print()
    print(predicate)
    print()
    print()

    predicate_path = failing_test_case + ".predicate.sh"
    with open(predicate_path, "w") as p:
        p.write(predicate)
    os.chmod(predicate_path, 0o755)

    creduce_command = ["creduce", "--n", str(os.cpu_count()), predicate_path, failing_test_case]
    print("Running:", creduce_command)
    result = run(creduce_command)
    if result.returncode == 0:
        print()
        print()
        print("`creduce` reduced the failing test case to:")
        print()
        print()
        cat(failing_test_case)
        print()
        print()
    else:
        print()
        print()
        print("`creduce` failed!")
        if not args.keep_going:
            sys.exit(1)

def print_issue_template(args, failing_test_case, predicate_command, result):
    test_case_contents = None
    with open(failing_test_case, "r") as f:
        test_case_contents = f.read()

    print("""

! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! !
! File this issue at https://github.com/rust-lang-nursery/rust-bindgen/issues/new !
! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! !

   --------------- 8< --------------- 8< --------------- 8< ---------------

This bug was found with `csmith` and `driver.py`.

### Input Header

```c
{}
```

### `bindgen` Invocation

```
$ {}
```

### Actual Results

<details>

```
{}
```

</details>

### Expected Results

`bindgen` emits bindings OK, then `rustc` compiles those bindings OK, then the
compiled bindings' layout tests pass OK.

   --------------- 8< --------------- 8< --------------- 8< ---------------

                         <3 <3 <3 Thank you! <3 <3 <3
    """.format(
        test_case_contents,
        " ".join(map(lambda s: "'{}'".format(s), predicate_command)),
        result.stdout + result.stderr
    ))

if __name__ == "__main__":
    try:
        os.chdir(os.path.abspath(os.path.dirname(sys.argv[0])))
        main()
    except KeyboardInterrupt:
        exit()
