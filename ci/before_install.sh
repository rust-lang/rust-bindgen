set -ex
pushd ~

# Workaround for Travis CI macOS bug (https://github.com/travis-ci/travis-ci/issues/6307)
if [ "${TRAVIS_OS_NAME}" == "osx" ]; then
    rvm get head || true
fi

function llvm_download_if_needed() {
    export LLVM_VERSION_TRIPLE="${LLVM_VERSION}"
    export LLVM=clang+llvm-${LLVM_VERSION_TRIPLE}-x86_64-$1

    local llvm_build_dir="$HOME/.llvm-builds/${LLVM}"

    if [ -d "${llvm_build_dir}" ]; then
        echo "Using cached LLVM build for ${LLVM} in ${llvm_build_dir}";
    else
        wget http://llvm.org/releases/${LLVM_VERSION_TRIPLE}/${LLVM}.tar.xz
        mkdir -p "${llvm_build_dir}"
        tar -xf ${LLVM}.tar.xz -C "${llvm_build_dir}" --strip-components=1
    fi

    export LLVM_CONFIG_PATH="${llvm_build_dir}/bin/llvm-config"
    if [ "${TRAVIS_OS_NAME}" == "osx" ]; then
        cp "${llvm_build_dir}/lib/libclang.dylib" /usr/local/lib/libclang.dylib
    fi
}


if [ "${TRAVIS_OS_NAME}" == "linux" ]; then
    llvm_download_if_needed linux-gnu-ubuntu-14.04
else
    llvm_download_if_needed apple-darwin
fi

popd
set +e
