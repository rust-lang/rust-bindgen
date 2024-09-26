# Requirements

This page lists the requirements for running `bindgen` and how to get them.

## Clang

`bindgen` leverages `libclang` to preprocess, parse, and type check C and C++
header files.

It is required to use Clang 9.0 or greater.

### Installing Clang

#### Windows

If you use winget:
```powershell
winget install LLVM.LLVM
```

Alternatively, you can download and install the official pre-built binary from
[LLVM download page](http://releases.llvm.org/download.html).

You will also need to set `LIBCLANG_PATH` as an [environment
variable](https://www.techjunkie.com/environment-variables-windows-10/) pointing
to the `bin` directory of your LLVM install. For example, if you installed LLVM
to `D:\programs\LLVM`, then you'd set the value to be `D:\programs\LLVM\bin`.

Alternatively, for Mingw64, you can install clang via
```bash
pacman -S  mingw64/mingw-w64-x86_64-clang
```

#### macOS

If you use Homebrew:

```bash
$ brew install llvm
```

If you use MacPorts:

```bash
$ port install clang
```

#### Debian-based Linuxes

```bash
# apt install libclang-dev
```

If you want to use the function `bindgen::Builder::dump_preprocessed_input`, then you also need the package `clang`.

#### Arch

```bash
# pacman -S clang
```

#### Fedora

```bash
# dnf install clang-devel
```

#### OpenBSD

```bash
# pkg_add llvm
```

Add `export LIBCLANG_PATH=/usr/local/lib` to your profile.

#### From source

If your package manager doesn't yet offer Clang 5.0, you'll need to build from
source. For that, follow the
instructions [here](http://clang.llvm.org/get_started.html).

Those instructions list optional steps. For `bindgen`:

* Checkout and build clang
* Checkout and build the extra-clang-tools
* You do not need to checkout or build compiler-rt
* You do not need to checkout or build libcxx
