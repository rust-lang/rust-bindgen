# Dockerfiles for running cargo test on bindgen-tests

## Dockerfile-clang

LLVM_VERSION can be [13, 18] and correspond to the packages at [apt.llvm.org](https://apt.llvm.org/) for Ubuntu 22.04

From the `rust-bingen` repo,

```
docker build dockerfiles -f dockerfiles/Dockerfile-clang -t clang:14-ubuntu --build-arg LLVM_VERSION=14
```

## Dockerfile-bindgen

From the `rust-bindgen` repo,

```
docker build . -f dockerfiles/Dockerfile-bindgen -t bindgen:clang-14-ubuntu --build-arg LLVM_VERSION=14
```

## Make changes to the repo happen in the docker container

```
docker run -v .:/project bindgen:clang-14-ubuntu
```

To pass an environment variable to the container

```
docker run -v .:/project -e BINDGEN_OVERWRITE_EXPECTED=1 bindgen:clang-14-ubuntu
```
