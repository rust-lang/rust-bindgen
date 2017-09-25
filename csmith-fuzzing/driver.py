from subprocess import run, DEVNULL, PIPE

csmith_command = [
        "csmith",
        "--no-checksum",
        "--nomain",
        "--max-block-size", "1",
        "--max-block-depth", "1",
        "--output", "generated.h"]

bindgen_command = ["bindgen", "generated.h"]

if __name__ == "__main__":
    print("Bindgen fuzzing with csmith.")
    print(
        "This script will write to generated.h, bindgen_stdout, bindgen_stderr and platform.info . "
        "These files can be deleted after running.")

    iterations = 0
    while True:
        print("\rIteration: {}".format(iterations), end="", flush=True)

        run(csmith_command, stdin=DEVNULL, stdout=DEVNULL, stderr=DEVNULL)
        with open("bindgen_stdout", "wb") as stdout, open("bindgen_stdout", "wb") as stderr:
            result = run(bindgen_command, stdin=DEVNULL, stdout=stdout, stderr=stderr)
            if result.returncode != 0:
                print()
                print(
                    "Error: bindgen existed with non zero exit code {} when ran on generated.h . "
                    "You can find its output in bindgen_stoud and bindgen_stderr."
                    .format(result.returncode))
                exit()
        iterations += 1
