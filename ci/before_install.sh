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
        5.*)                echo "linux-x86_64-ubuntu14.04" ;;
        *)                  echo "x86_64-linux-gnu-ubuntu-14.04" ;;
    esac
}

function llvm_macos_target_triple() {
    case "$1" in
        [0-8].* | 9.0.0)    echo "x86_64-darwin-apple" ;;
        # Starting with 9.0.1, triple swapped ordering
        *)                  echo "x86_64-apple-darwin" ;;
    esac
}

function llvm_version_triple() {
    case "$1" in
        3.5) echo "3.5.2" ;;
        3.6) echo "3.6.2" ;;
        3.7) echo "3.7.1" ;;
        3.8) echo "3.8.1" ;;
        # By default, take the .0 patch release
        *)   echo "$1.0"  ;;
    esac
}

function llvm_base_url() {
    local llvm_version_triple=$1

    case "$llvm_version_triple" in
        [0-8].* | 9.0.0)
            echo "http://releases.llvm.org/$llvm_version_triple"
            ;;
        # Starting with 9.0.1, releases are hosted on github
        *)
            echo "https://github.com/llvm/llvm-project/releases/download/llvmorg-$llvm_version_triple"
            ;;
    esac
}

function llvm_download() {
    local base_url=$1
    local arch=$2

    export LLVM=clang+llvm-${LLVM_VERSION_TRIPLE}-$arch
    export LLVM_DIRECTORY="$HOME/.llvm/${LLVM}"

    local base_url

    if [ -d "${LLVM_DIRECTORY}" ]; then
        echo "Using cached LLVM download for ${LLVM}..."
    else
        wget $base_url/${LLVM}.tar.xz
        mkdir -p "${LLVM_DIRECTORY}"
        tar xf ${LLVM}.tar.xz -C "${LLVM_DIRECTORY}" --strip-components=1
    fi

    export LIBCLANG_PATH="${LLVM_DIRECTORY}/lib"
    export LLVM_CONFIG_PATH="${LLVM_DIRECTORY}/bin/llvm-config"
}

export LLVM_VERSION_TRIPLE=`llvm_version_triple ${LLVM_VERSION}`

base_url=`llvm_base_url ${LLVM_VERSION_TRIPLE}`

if [ "${TRAVIS_OS_NAME}" == "linux" ]; then
    llvm_download $base_url `llvm_linux_target_triple ${LLVM_VERSION_TRIPLE}`
    export LD_LIBRARY_PATH="${LLVM_DIRECTORY}/lib":${LD_LIBRARY_PATH:-}
else
    llvm_download $base_url `llvm_macos_target_triple ${LLVM_VERSION_TRIPLE}`
    export DYLD_LIBRARY_PATH="${LLVM_DIRECTORY}/lib":${DYLD_LIBRARY_PATH:-}
fi

# Subsequent scripts can see the state of `set -eu`, so unset it again.
set +eu
