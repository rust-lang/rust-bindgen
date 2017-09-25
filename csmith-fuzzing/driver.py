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

def run_logged(cmd):
    with NamedTemporaryFile() as stdout, NamedTemporaryFile() as stderr:
        result = run(cmd, stdin=DEVNULL, stdout=stdout, stderr=stderr)
        if result.returncode != 0:
            print()
            print("Error: {} exited with code {}".format(str(cmd), result.returncode))
            print("-------------------- stdout --------------------")
            run(["cat", stdout.name])
            print("-------------------- stderr --------------------")
            run(["cat", stderr.name])
        return result

def run_bindgen(input, output):
    return run_logged([
        "bindgen",
        "--with-derive-partialeq",
        "--with-derive-eq",
        "-o", output.name,
        input.name,
        "--",
        "-I", os.path.abspath(os.path.dirname(sys.argv[0])),
    ])

def main():
    print("Fuzzing `bindgen` with C-Smith...\n")

    iterations = 0
    while True:
        print("\rIteration: {}".format(iterations), end="", flush=True)

        input = NamedTemporaryFile(delete=False, prefix="input-", suffix=".h")
        result = run_logged(csmith_command + ["-o", input.name])
        if result.returncode != 0:
            exit(1)

        output = NamedTemporaryFile(delete=False, prefix="output-", suffix=".rs")
        result = run_bindgen(input, output)
        if result.returncode != 0:
            print("-------------------- {} --------------------".format(input.name))
            run(["cat", input.name])
            print("-------------------- {} --------------------".format(output.name))
            run(["cat", output.name])
            exit(1)

        os.remove(input.name)
        os.remove(output.name)

        iterations += 1

if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        exit()
