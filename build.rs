extern crate cmake;

mod target {
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::path::{Path, PathBuf};

    pub fn main() {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

        let mut dst =
            File::create(Path::new(&out_dir).join("host-target.txt")).unwrap();
        dst.write_all(env::var("TARGET").unwrap().as_bytes())
            .unwrap();
    }
}

mod testgen {
    use std::char;
    use std::env;
    use std::ffi::OsStr;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::{Path, PathBuf};

    pub fn main() {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        let mut dst =
            File::create(Path::new(&out_dir).join("tests.rs")).unwrap();

        let manifest_dir =
            PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let headers_dir = manifest_dir.join("tests").join("headers");

        let headers = match fs::read_dir(headers_dir) {
            Ok(dir) => dir,
            // We may not have headers directory after packaging.
            Err(..) => return,
        };

        let entries =
            headers.map(|result| result.expect("Couldn't read header file"));

        println!("cargo:rerun-if-changed=tests/headers");

        for entry in entries {
            match entry.path().extension().and_then(OsStr::to_str) {
                Some("h") | Some("hpp") => {
                    let func = entry
                        .file_name()
                        .to_str()
                        .unwrap()
                        .replace(|c| !char::is_alphanumeric(c), "_")
                        .replace("__", "_")
                        .to_lowercase();
                    writeln!(
                        dst,
                        "test_header!(header_{}, {:?});",
                        func,
                        entry.path(),
                    )
                    .unwrap();
                }
                _ => {}
            }
        }

        dst.flush().unwrap();
    }
}

mod clang_ast {
    use std::env;
    use std::ffi::OsStr;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::{Command, Stdio};

    pub fn main() {
        let llvm_info = LLVMInfo::new();
        build_native(&llvm_info);
    }

    fn build_var(name: &str) -> Option<String> {
        println!("cargo:rerun-if-env-changed={}", name);
        env::var(name).ok()
    }

    /// Strip full path from library if provided. rustc expects us to
    /// pass just the library name in `-l` arguments.
    fn clean_lib_path(lib: &str) -> String {
        if lib.starts_with("-l") {
            lib[2..].to_string()
        } else {
            // Sometimes llvm-config gives us an absolute path
            // to the library, and I can't figure out a way to
            // give an absolute path of a library to rustc.
            Path::new(lib)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .trim_start_matches("lib")
                .into()
        }
    }

    /// Call out to CMake, build the clang ast library, and tell cargo where to look
    /// for it.  Note that `CMAKE_BUILD_TYPE` gets implicitly determined by the
    /// cmake crate according to the following:
    ///
    ///   - if `opt-level=0`                              then `CMAKE_BUILD_TYPE=Debug`
    ///   - if `opt-level={1,2,3}` and not `debug=false`, then `CMAKE_BUILD_TYPE=RelWithDebInfo`
    fn build_native(llvm_info: &LLVMInfo) {
        // Find where the (already built) LLVM lib dir is
        let llvm_lib_dir = &llvm_info.lib_dir;
        let mut llvm_cmake_dir: PathBuf = llvm_lib_dir.into();
        llvm_cmake_dir.push("cmake");
        llvm_cmake_dir.push("llvm");
        let mut clang_cmake_dir: PathBuf = llvm_lib_dir.into();
        clang_cmake_dir.push("cmake");
        clang_cmake_dir.push("clang");

        if let Some((major_version, minor_version)) = llvm_info.version {
            if (major_version == 3 && minor_version <= 8) || major_version < 3 {
                llvm_cmake_dir = PathBuf::from(llvm_lib_dir);
                llvm_cmake_dir.pop();
                llvm_cmake_dir.push("share");
                llvm_cmake_dir.push("llvm");
                llvm_cmake_dir.push("cmake");
                clang_cmake_dir = PathBuf::from(llvm_lib_dir);
                clang_cmake_dir.pop();
                clang_cmake_dir.push("share");
                clang_cmake_dir.push("clang");
                clang_cmake_dir.push("cmake");
            }
        }

        println!("cargo:rerun-if-changed=src/clang/clang_interface.hpp");
        println!("cargo:rerun-if-changed=src/clang/clang_interface_impl.hpp");
        println!("cargo:rerun-if-changed=src/clang/clang_interface.cpp");
        println!("cargo:rerun-if-changed=src/clang/libclang_compat.cpp");
        // Build libBindgenClangInterface.a with cmake
        let out_dir = cmake::Config::new("src/clang")
            .define("LLVM_DIR", llvm_cmake_dir.as_os_str())
            .define("Clang_DIR", clang_cmake_dir.as_os_str())
            .build();

        // Set up search path for newly built libBindgenClangInterface.a
        let lib_dir = out_dir.join("lib");
        println!("cargo:rustc-link-search=native={}", lib_dir.display());

        // Statically link against our library, 'BindgenClangInterface'
        println!("cargo:rustc-link-lib=static=BindgenClangInterface");

        let deps_filepath = out_dir.join("BindgenClangInterface.deps");
        println!("cargo:rustc-link-search=native={}", llvm_lib_dir);
        if deps_filepath.is_file() {
            // Our CMake script was able to generate a list of dependencies for
            // us. This should be more accurate than what we build here.
            let deps_file = fs::read_to_string(deps_filepath)
                .expect("Could not read deps file");
            let deps = deps_file
                .split(";")
                .filter(|dep| {
                    // We're skipping any dependencies with delayLoad, because
                    // rustc doesn't know how to handle these. We don't seem to
                    // need them anyway.
                    !dep.starts_with("-delayload")
                })
                .map(clean_lib_path);
            for lib in deps {
                println!("cargo:rustc-link-lib={}", lib);
            }
        } else {
            // Link against these Clang libs. The ordering here is important! Libraries
            // must be listed before their dependencies when statically linking.
            for lib in &[
                "clangIndex",
                "clangFrontend",
                "clangParse",
                "clangSerialization",
                "clangSema",
                "clangEdit",
                "clangAnalysis",
                "clangDriver",
                "clangFormat",
                "clangToolingCore",
                "clangAST",
                "clangLex",
                "clangBasic",
            ] {
                println!("cargo:rustc-link-lib={}", lib);
            }

            for lib in &llvm_info.libs {
                // IMPORTANT: We cannot specify static= or dylib= here because rustc
                // will reorder those libs before the clang libs above which don't have
                // static or dylib.
                println!("cargo:rustc-link-lib={}", lib);
            }

            // Link against the C++ std library.
            if cfg!(target_os = "macos") {
                println!("cargo:rustc-link-lib=c++");
            } else if cfg!(not(target_os = "windows")) {
                println!("cargo:rustc-link-lib=stdc++");
            }

            // clangDriver links against system version.lib on windows
            if cfg!(target_os = "windows") {
                println!("cargo:rustc-link-lib=version")
            }
        }
    }

    /// Holds information about LLVM paths we have found
    struct LLVMInfo {
        /// LLVM lib dir containing libclang* and libLLVM* libraries
        pub lib_dir: String,

        /// List of libs we need to link against
        pub libs: Vec<String>,

        /// LLVM version in (major, minor) form, if available
        pub version: Option<(i32, i32)>,
    }

    impl LLVMInfo {
        fn new() -> Self {
            fn find_llvm_config() -> Option<String> {
                // Explicitly provided path in LLVM_CONFIG_PATH
                build_var("LLVM_CONFIG_PATH")
                    // Relative to LLVM_LIB_DIR
                    .or_else(|| {
                        build_var("LLVM_LIB_DIR").map(|d| {
                            String::from(
                                Path::new(&d)
                                    .join("../bin/llvm-config")
                                    .canonicalize()
                                    .unwrap()
                                    .to_string_lossy(),
                            )
                        })
                    })
                    // In PATH
                    .or_else(|| {
                        [
                            "llvm-config-7.0",
                            "llvm-config-6.1",
                            "llvm-config-6.0",
                            "llvm-config",
                            // Homebrew install location on MacOS
                            "/usr/local/opt/llvm/bin/llvm-config",
                        ]
                        .iter()
                        .find_map(|c| {
                            if Command::new(c)
                                .stdout(Stdio::null())
                                .stderr(Stdio::null())
                                .spawn()
                                .is_ok()
                            {
                                Some(String::from(*c))
                            } else {
                                None
                            }
                        })
                    })
            }

            /// Invoke given `command`, if any, with the specified arguments.
            fn invoke_command<I, S, C>(
                command: Option<C>,
                args: I,
            ) -> Option<String>
            where
                I: IntoIterator<Item = S>,
                S: AsRef<OsStr>,
                C: AsRef<OsStr>,
            {
                command.and_then(|c| {
                    Command::new(c).args(args).output().ok().and_then(
                        |output| {
                            if output.status.success() {
                                Some(
                                    String::from_utf8_lossy(&output.stdout)
                                        .trim()
                                        .to_string(),
                                )
                            } else {
                                None
                            }
                        },
                    )
                })
            }

            let llvm_config = find_llvm_config();
            let lib_dir = {
                let path_str = build_var("LLVM_LIB_DIR")
                    .or(invoke_command(llvm_config.as_ref(), &["--libdir"]))
                    .expect(
                        "
Couldn't find LLVM lib dir. Try setting the `LLVM_LIB_DIR` environment
variable or make sure `llvm-config` is on $PATH then re-build. For example:

  $ export LLVM_LIB_DIR=/usr/local/opt/llvm/lib
",
                    );
                String::from(
                    Path::new(&path_str)
                        .canonicalize()
                        .unwrap()
                        .to_string_lossy(),
                )
            };

            let llvm_shared_libs = invoke_command(
                llvm_config.as_ref(),
                &["--libs", "--link-shared"],
            );

            // <sysroot>/lib/rustlib/<target>/lib/ contains a libLLVM DSO for the
            // rust compiler. On MacOS, this lib is named libLLVM.dylib, which will
            // always conflict with the dylib we are trying to link against. On
            // Linux we generally will not hit this issue because the prebuilt lib
            // includes the `svn` suffix. This would conflict with a source build
            // from master, however.
            //
            // We check here if the lib we want to link against will conflict with
            // the rustlib version. If so we can't dynamically link against libLLVM.
            let conflicts_with_rustlib_llvm = {
                if let Some(llvm_shared_libs) = llvm_shared_libs.as_ref() {
                    let dylib_suffix = {
                        if cfg!(target_os = "macos") {
                            ".dylib"
                        } else {
                            ".so"
                        } // Windows is not supported
                    };
                    let mut dylib_file = String::from("lib");
                    dylib_file
                        .push_str(llvm_shared_libs.trim_start_matches("-l"));
                    dylib_file.push_str(dylib_suffix);
                    let sysroot = invoke_command(
                        env::var("RUSTC").ok().as_ref(),
                        &["--print=sysroot"],
                    )
                    .unwrap();

                    // Does <sysroot>/lib/rustlib/<target>/lib/<dylib_file> exist?
                    let mut libllvm_path = PathBuf::new();
                    libllvm_path.push(sysroot);
                    libllvm_path.push("lib/rustlib");
                    libllvm_path.push(env::var("TARGET").unwrap());
                    libllvm_path.push("lib");
                    libllvm_path.push(dylib_file);

                    libllvm_path.as_path().exists()
                } else {
                    false
                }
            };

            let link_statically = cfg!(feature = "llvm-static") || {
                let args = if conflicts_with_rustlib_llvm {
                    vec!["--shared-mode", "--ignore-libllvm"]
                } else {
                    vec!["--shared-mode"]
                };
                invoke_command(llvm_config.as_ref(), &args)
                    .map_or(false, |c| c == "static")
            };

            let link_mode = if link_statically {
                "--link-static"
            } else {
                "--link-shared"
            };

            let version_string = invoke_command(
                llvm_config.as_ref(),
                &["--version"],
            )
            .or_else(|| {
                invoke_command(
                    Some(&format!("{}/../bin/clang", lib_dir)),
                    &["--version"],
                )
                .and_then(|output| Some(output.split(" ").nth(2)?.to_string()))
            });
            let version = version_string.and_then(|version| {
                let mut split = version.split(".");
                let major: i32 = split.next()?.parse().ok()?;
                let minor: i32 = split.next()?.parse().ok()?;
                Some((major, minor))
            });

            let mut supports_link_mode = true;
            if let Some((major_version, minor_version)) = version {
                if major_version < 3
                    || (major_version == 3 && minor_version <= 8)
                {
                    supports_link_mode = false;
                }
            }

            // LLVM components that we need to link against for the clang libs
            let mut llvm_components = vec![
                "MC",
                "MCParser",
                "Support",
                "Option",
                "BitReader",
                "ProfileData",
                "Core",
            ];

            // llvmAST requires FrontendOpenMP from version 10 and newer
            if let Some((version, _minor_version)) = version {
                if version > 4 {
                    llvm_components.push("BinaryFormat");
                }
                if version > 9 {
                    llvm_components.push("FrontendOpenMP");
                }
            }

            // Construct the list of libs we need to link against
            let mut args = llvm_components;
            args.insert(0, "--libs");
            if supports_link_mode {
                args.insert(1, link_mode);
            }
            let mut libs: Vec<String> =
                invoke_command(llvm_config.as_ref(), &args)
                    .unwrap_or("-lLLVM".to_string())
                    .split_whitespace()
                    .map(clean_lib_path)
                    .collect();

            libs.extend(
                build_var("LLVM_SYSTEM_LIBS")
                    .or_else(|| {
                        let mut args = vec!["--system-libs"];
                        if supports_link_mode {
                            args.push(link_mode);
                        }
                        invoke_command(llvm_config.as_ref(), args)
                    })
                    .unwrap_or(String::new())
                    .split_whitespace()
                    .map(clean_lib_path),
            );

            Self {
                lib_dir,
                libs,
                version,
            }
        }
    }
}

fn main() {
    target::main();
    testgen::main();
    clang_ast::main()
}
