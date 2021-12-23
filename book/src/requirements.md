# Requirements

This page lists the requirements for running `bindgen` and how to get them.

## Clang

`bindgen` leverages `libclang` to preprocess, parse, and type check C and C++
header files.

It is recommended to use Clang 3.9 or greater, however `bindgen` can run with
older Clangs with some features disabled.

* **If you are generating bindings to C,** 3.7 and 3.8 will probably work OK for
you.

* **If you are generating bindings to C++,** you almost definitely want 3.9 or
greater.

### Installing Clang 3.9

#### Windows

Download and install the official pre-built binary from
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
# apt install llvm-dev libclang-dev clang
```

Ubuntu 16.10 provides the necessary packages directly. If you are using older
version of Ubuntu or other Debian-based distros, you may need to add the LLVM
repos to get version 3.9. See http://apt.llvm.org/.

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

If your package manager doesn't yet offer Clang 3.9, you'll need to build from
source. For that, follow the
instructions [here](http://clang.llvm.org/get_started.html).

Those instructions list optional steps. For `bindgen`:

* Checkout and build clang
* Checkout and build the extra-clang-tools
* You do not need to checkout or build compiler-rt
* You do not need to checkout or build libcxx
