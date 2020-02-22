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

    /// Call out to CMake, build the clang ast library, and tell cargo where to look
    /// for it.  Note that `CMAKE_BUILD_TYPE` gets implicitly determined by the
    /// cmake crate according to the following:
    ///
    ///   - if `opt-level=0`                              then `CMAKE_BUILD_TYPE=Debug`
    ///   - if `opt-level={1,2,3}` and not `debug=false`, then `CMAKE_BUILD_TYPE=RelWithDebInfo`
    fn build_native(llvm_info: &LLVMInfo) {
        // Find where the (already built) LLVM lib dir is
        let llvm_lib_dir = &llvm_info.lib_dir;

        println!("cargo:rerun-if-changed=src/clang/ClangAST.cpp");
        println!("cargo:rerun-if-changed=src/clang/ClangAST.hpp");
        // Build libbindgenClangAST.a with cmake
        let dst = cmake::Config::new("src/clang")
            .define("LLVM_DIR", &format!("{}/cmake/llvm", llvm_lib_dir))
            .define("Clang_DIR", &format!("{}/cmake/clang", llvm_lib_dir))
            .build_target("bindgenClangAST")
            .build();

        let out_dir = dst.display();

        // Set up search path for newly built libbindgenClangAST.a
        println!("cargo:rustc-link-search=native={}/build/lib", out_dir);
        println!("cargo:rustc-link-search=native={}/build", out_dir);

        // Statically link against our library, 'bindgenClangAST'
        println!("cargo:rustc-link-lib=static=bindgenClangAST");

        // Link against these Clang libs. The ordering here is important! Libraries
        // must be listed before their dependencies when statically linking.
        println!("cargo:rustc-link-search=native={}", llvm_lib_dir);
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
        } else {
            println!("cargo:rustc-link-lib=stdc++");
        }
    }

    /// Holds information about LLVM paths we have found
    struct LLVMInfo {
        /// LLVM lib dir containing libclang* and libLLVM* libraries
        pub lib_dir: String,

        /// List of libs we need to link against
        pub libs: Vec<String>,
    }

    impl LLVMInfo {
        fn new() -> Self {
            fn find_llvm_config() -> Option<String> {
                // Explicitly provided path in LLVM_CONFIG_PATH
                build_var("LLVM_CONFIG_PATH")
                // Relative to LLVM_LIB_DIR
                    .or(build_var("LLVM_LIB_DIR").map(|d| {
                        String::from(
                            Path::new(&d)
                                .join("../bin/llvm-config")
                                .canonicalize()
                                .unwrap()
                                .to_string_lossy(),
                        )
                    }))
                // In PATH
                    .or([
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
                        }))
            }

            /// Invoke given `command`, if any, with the specified arguments.
            fn invoke_command<I, S, C>(command: Option<C>, args: I) -> Option<String>
                where
                I: IntoIterator<Item = S>,
                S: AsRef<OsStr>,
                C: AsRef<OsStr>,
            {
                command.and_then(|c| {
                    Command::new(c).args(args).output().ok().and_then(|output| {
                        if output.status.success() {
                            Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
                        } else {
                            None
                        }
                    })
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

            let llvm_shared_libs = invoke_command(llvm_config.as_ref(), &["--libs", "--link-shared"]);

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
                    dylib_file.push_str(llvm_shared_libs.trim_start_matches("-l"));
                    dylib_file.push_str(dylib_suffix);
                    let sysroot =
                        invoke_command(env::var("RUSTC").ok().as_ref(), &["--print=sysroot"]).unwrap();

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
                invoke_command(llvm_config.as_ref(), &args).map_or(false, |c| c == "static")
            };

            let link_mode = if link_statically {
                "--link-static"
            } else {
                "--link-shared"
            };

            let llvm_version = invoke_command(llvm_config.as_ref(), &["--version"]);

            let llvm_major_version = llvm_version
                .and_then(|version| {
                    let major: i32 = version.split(".").next()?.parse().ok()?;
                    Some(major)
                });

            // LLVM components that we need to link against for the clang libs
            let mut llvm_components = vec![
                "MC",
                "MCParser",
                "Support",
                "Option",
                "BitReader",
                "ProfileData",
                "BinaryFormat",
                "Core",
            ];

            // Construct the list of libs we need to link against
            let mut args = llvm_components;
            args.insert(0, "--libs");
            args.insert(1, link_mode);
            let mut libs: Vec<String> = invoke_command(llvm_config.as_ref(), &args)
                .unwrap_or("-lLLVM".to_string())
                .split_whitespace()
                .map(|lib| String::from(lib.trim_start_matches("-l")))
                .collect();

            libs.extend(
                build_var("LLVM_SYSTEM_LIBS")
                    .or(invoke_command(
                        llvm_config.as_ref(),
                        &[
                            "--system-libs",
                            link_mode,
                        ],
                    ))
                    .unwrap_or(String::new())
                    .split_whitespace()
                    .map(|lib| {
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
                    })
            );

            Self {
                lib_dir,
                libs,
            }
        }
    }
}

fn main() {
    target::main();
    testgen::main();
    clang_ast::main()
}
