import os, sys
from subprocess import run, SubprocessError, DEVNULL, PIPE
from tempfile import NamedTemporaryFile

csmith_command = [
    "csmith",
    "--no-checksum",
    "--nomain",
    "--max-block-size", "1",
    "--max-block-depth", "1",
]

def cat(path, title=None):
    if not title:
        title = path
    print("-------------------- {} --------------------".format(title))
    run(["cat", path])

def run_logged(cmd):
    with NamedTemporaryFile() as stdout, NamedTemporaryFile() as stderr:
        result = run(cmd, stdin=DEVNULL, stdout=stdout, stderr=stderr)
        if result.returncode != 0:
            print()
            print("Error: '{}' exited with code {}".format(" ".join(cmd), result.returncode))
            cat(stdout.name, title="stdout")
            cat(stdout.name, title="stderr")
        return result

def run_bindgen(input, output):
    return run_logged([
        "bindgen",
        "--with-derive-partialeq",
        "--with-derive-eq",
        "--with-derive-partialord",
        "--with-derive-ord",
        "--with-derive-hash",
        "--with-derive-default",
        "-o", output.name,
        input.name,
        "--",
        "-I", os.path.abspath(os.path.dirname(sys.argv[0])),
    ])

def run_rustc(output, test):
    return run_logged([
        "rustc",
        "--crate-type", "lib",
        "--test",
        output.name,
        "-o", test.name,
    ])

def main():
    print("Fuzzing `bindgen` with C-Smith...\n")

    iterations = 0
    while True:
        print("\rIteration: {}".format(iterations), end="", flush=True)

        input = NamedTemporaryFile(delete=False, prefix="input-", suffix=".h")
        input.close()
        result = run_logged(csmith_command + ["-o", input.name])
        if result.returncode != 0:
            exit(1)

        output = NamedTemporaryFile(delete=False, prefix="output-", suffix=".rs")
        output.close()
        result = run_bindgen(input, output)
        if result.returncode != 0:
            cat(input.name)
            cat(output.name)
            exit(1)

        test = NamedTemporaryFile(delete=False, prefix="test-")
        test.close()
        result = run_rustc(output, test)
        if result.returncode != 0:
            cat(input.name)
            cat(output.name)
            exit(1)

        result = run_logged([test.name])
        if result.returncode != 0:
            cat(input.name)
            cat(output.name)
            exit(1)

        os.remove(input.name)
        os.remove(output.name)
        os.remove(test.name)

        iterations += 1

if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        exit()
