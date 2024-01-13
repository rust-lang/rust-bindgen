#!/usr/bin/env bash
# Bail on first error
set -e
# Bail if an unset variable is encountered
set -u
# Enable debugging output
set -x
# Give a pipeline a non-zero exit code if one of its constituents fails
set -o pipefail

function llvm_linux_target_triple() {
  case "$1" in
    9.0.1)  echo "x86_64-linux-gnu-ubuntu-16.04" ;;
    *)      echo "x86_64-linux-gnu-ubuntu-18.04" ;;
  esac
}

function llvm_macos_target_triple() {
  case "$1" in
    9.0.1)  echo "x86_64-apple-darwin" ;;
    *)      echo "arm64-apple-darwin22.0" ;;
  esac
}

function llvm_version_triple() {
  case "$1" in
    9.0) echo "9.0.1" ;;
    # By default, take the .0 patch release
    *)   echo "$1.0"  ;;
  esac
}

function llvm_base_url() {
  local llvm_version_triple=$1
  echo "https://github.com/llvm/llvm-project/releases/download/llvmorg-$llvm_version_triple"
}

function llvm_download() {
  local base_url=$1
  local arch=$2

  export LLVM=clang+llvm-${LLVM_VERSION_TRIPLE}-$arch
  export LLVM_DIRECTORY="$HOME/.llvm/${LLVM}"

  if [ -d "${LLVM_DIRECTORY}" ]; then
    echo "Using cached LLVM download for ${LLVM}..."
  else
    wget --no-verbose $base_url/${LLVM}.tar.xz
    mkdir -p "${LLVM_DIRECTORY}"
    tar xf ${LLVM}.tar.xz -C "${LLVM_DIRECTORY}" --strip-components=1
  fi

  export LIBCLANG_PATH="${LLVM_DIRECTORY}/lib"
  export LLVM_CONFIG_PATH="${LLVM_DIRECTORY}/bin/llvm-config"
}

# Download and set up a sane LLVM version
set_llvm_env() {
  export LLVM_VERSION_TRIPLE=`llvm_version_triple ${LLVM_VERSION}`
  local base_url=`llvm_base_url ${LLVM_VERSION_TRIPLE}`

  if [ "$GITHUB_ACTIONS_OS" == "ubuntu-latest" ]; then
    llvm_download $base_url `llvm_linux_target_triple ${LLVM_VERSION_TRIPLE}`
    export LD_LIBRARY_PATH="${LLVM_DIRECTORY}/lib":${LD_LIBRARY_PATH:-}
  else
    llvm_download $base_url `llvm_macos_target_triple ${LLVM_VERSION_TRIPLE}`
    export DYLD_LIBRARY_PATH="${LLVM_DIRECTORY}/lib":${DYLD_LIBRARY_PATH:-}
  fi
}

assert_no_diff() {
  git add -u
  git diff @
  git diff-index --quiet HEAD
}

set_llvm_env

get_cargo_args() {
  local args=""
  if [ ! -z "$RUST_TARGET" ]; then
    args+=" --target $RUST_TARGET"
  fi
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

if [ ! -z "$RUST_CROSS_COMPILER" ]; then
  export RUSTFLAGS="-C linker=${RUST_CROSS_COMPILER}-gcc"
fi

CARGO_ARGS=`get_cargo_args`

# Ensure we build without warnings
RUSTFLAGS="-Dwarnings" cargo check $CARGO_ARGS

if [ "$BINDGEN_MAIN_TESTS" == "1" ]; then
  # Run the tests
  (cd bindgen-tests && cargo test $CARGO_ARGS)
fi

assert_no_diff

# Run the integration tests
(cd bindgen-integration && cargo test $CARGO_ARGS)
