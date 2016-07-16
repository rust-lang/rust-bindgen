set -e

function llvm_version_triple() {
    if [ "$1" == "3.5" ]; then
        echo "3.5.2"
    elif [ "$1" == "3.6" ]; then
        echo "3.6.2"
    elif [ "$1" == "3.7" ]; then
        echo "3.7.1"
    elif [ "$1" == "3.8" ]; then
        echo "3.8.0"
    fi
}

function linux() {
    export LLVM_VERSION_TRIPLE=`llvm_version_triple ${LLVM_VERSION}`
    export LLVM=clang+llvm-${LLVM_VERSION_TRIPLE}-x86_64-linux-gnu-ubuntu-14.04

    wget http://llvm.org/releases/${LLVM_VERSION_TRIPLE}/${LLVM}.tar.xz
    mkdir llvm
    tar -xf ${LLVM}.tar.xz -C llvm --strip-components=1

    export LLVM_CONFIG_PATH=`pwd`/llvm/bin/llvm-config
}

function osx() {
    if [ "${LLVM_VERSION}" == "devtools" ]; then
        export LIBCLANG_PATH=/Library/Developer/CommandLineTools/usr/lib;
    else
        brew update >/dev/null
        brew install llvm3${LLVM_VERSION#3.}

        export LLVM_CONFIG_PATH=`brew --prefix llvm3${LLVM_VERSION#3.}`/lib/llvm-${LLVM_VERSION}/bin/llvm-config
    fi
}

if [ "${TRAVIS_OS_NAME}" == "osx" ]; then osx; else linux; fi
