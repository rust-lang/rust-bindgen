Fuzz bindgen with `csmith` https://github.com/csmith-project/csmith .

Run with `python3 driver.py`. It will run until until it encounters an error in `bindgen`.

Requires `python3`, `csmith` and `bindgen` to be in `$PATH`.

csmith is run with `--no-checksum --nomain --max-block-size 1 --max-block-depth 1` which disables the `main` function and makes function bodies as simple as possible as bindgen does not care about them but they cannot be completely disabled in csmith. Run `csmith --help` to see what exactly those options do.
