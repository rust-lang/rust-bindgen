#!/usr/bin/env bash
# Bail on first error
set -e
# Bail if an unset variable is encountered
set -u
# Enable debugging output
set -x
# Give a pipeline a non-zero exit code if one of its constituents fails
set -o pipefail

set_llvm_env() {
  export LLVM_CONFIG_PATH=${LLVM_PATH}/bin/llvm-config
  echo "LLVM_CONFIG_PATH=$LLVM_CONFIG_PATH"
  
  export LIBCLANG_PATH=${LLVM_PATH}/lib/
  echo "LIBCLANG_PATH=$LIBCLANG_PATH"

  export CLANG_PATH=${LLVM_PATH}/bin/clang
  echo "CLANG_PATH=$CLANG_PATH"
}

assert_no_diff() {
  git add -u
  git diff @
  git diff-index --quiet HEAD
}

get_cargo_args() {
  local args=""
  if [ "$BINDGEN_RELEASE_BUILD" == "1" ]; then
    args+=" --release"
  fi
  if [ "$BINDGEN_NO_DEFAULT_FEATURES" == "1" ]; then
    args+=" --no-default-features"
  fi
  local features=""
  if [ "$BINDGEN_FEATURE_RUNTIME" == "1"  ]; then
    features+="runtime"
  fi
  if [ "$BINDGEN_FEATURE_EXTRA_ASSERTS" == "1"  ]; then
    features+=" __testing_only_extra_assertions"
  fi
  if [ ! -z "$features" ]; then
    args+=" --features $(echo $features | tr ' ' ',')"
  fi
  echo $args
}

set_llvm_env

CARGO_ARGS=`get_cargo_args`

# Ensure we build without warnings
RUSTFLAGS="-Dwarnings" cargo check $CARGO_ARGS

# Run the tests
(cd bindgen-tests && cargo test $CARGO_ARGS)

assert_no_diff

# Run the integration tests
(cd bindgen-integration && cargo test $CARGO_ARGS)

if [ "$BINDGEN_RUST_FOR_LINUX_TEST" == "1" ]; then
  # Run the Rust for Linux test
  #
  # This is intended to be an end-to-end test that runs what Linux kernel
  # developers/users would do. It is a quick, build-only test of the Rust code
  # in the Linux kernel.

  # Put LLVM binaries in the path for `LLVM=1`. The LLVM `bin` directory should
  # go first since there are others in the Ubuntu image.
  export PATH="${LLVM_PATH}/bin:${PATH}"

  # Kernel build dependency: `bindgen-cli`, which is under test.
  #
  # Using `cargo build` (and adding the two common profiles to the `$PATH`) so
  # that we can use `$CARGO_ARGS` as is, since `cargo install` does not support
  # `--release`. `--target-dir` is used to isolate from other possible tests.
  # A cleaner alternative is using `--out-dir`, but it is unstable.
  (cd bindgen-cli && cargo build --target-dir ${HOME}/.bindgen $CARGO_ARGS)
  export PATH="${HOME}/.bindgen/release:${HOME}/.bindgen/debug:${PATH}"

  # Kernel build dependency: `libelf-dev`.
  sudo apt-get update
  sudo apt-get install libelf-dev

  # Kernel build dependency: the Rust standard library sources.
  #
  # `rustup` is used here to install the `rust-src` component (instead of using
  # `actions-rs/toolchain`'s `components` option in the workflow step) since we
  # only need it for this test, and anyway the action installs `rustup`.
  rustup component add rust-src

  # Ideally this should be updated from time to time (e.g. every kernel `-rc1`),
  # and each update should only contain this change.
  #
  # Both commit hashes and tags are supported.
  LINUX_VERSION=v6.16-rc1

  # Download Linux at a specific commit
  mkdir -p linux
  git -C linux init
  git -C linux remote add origin https://github.com/torvalds/linux.git
  git -C linux fetch --depth 1 origin ${LINUX_VERSION}
  git -C linux checkout FETCH_HEAD

  # Configure Rust for Linux
  cat <<EOF > linux/kernel/configs/rfl-for-bindgen-ci.config
# CONFIG_WERROR is not set

CONFIG_RUST=y

CONFIG_SAMPLES=y
CONFIG_SAMPLES_RUST=y

CONFIG_SAMPLE_RUST_MINIMAL=m
CONFIG_SAMPLE_RUST_PRINT=y

CONFIG_RUST_PHYLIB_ABSTRACTIONS=y
CONFIG_AX88796B_PHY=y
CONFIG_AX88796B_RUST_PHY=y

CONFIG_KUNIT=y
CONFIG_RUST_KERNEL_DOCTESTS=y
EOF

  make -C linux LLVM=1 -j$(($(nproc) + 1)) \
      rustavailable \
      defconfig \
      rfl-for-bindgen-ci.config

  # Build Rust for Linux
  make -C linux LLVM=1 -j$(($(nproc) + 1)) \
      samples/rust/rust_minimal.o \
      samples/rust/rust_print_main.o \
      drivers/net/phy/ax88796b_rust.o
fi
