use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

const LINUX_CLANG_DIRS: &'static [&'static str] = &[
    "/usr/lib",
    "/usr/lib/llvm",
    "/usr/lib/llvm-3.8/lib",
    "/usr/lib/llvm-3.7/lib",
    "/usr/lib/llvm-3.6/lib",
    "/usr/lib/llvm-3.5/lib",
    "/usr/lib/llvm-3.4/lib",
    "/usr/lib64/llvm",
    "/usr/lib/x86_64-linux-gnu",
];
const MAC_CLANG_DIR: &'static [&'static str] = &[
    "/usr/local/opt/llvm/lib",
    "/Library/Developer/CommandLineTools/usr/lib",
    "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib",
];
const WIN_CLANG_DIRS: &'static [&'static str] = &["C:\\Program Files\\LLVM\\bin", "C:\\Program Files\\LLVM\\lib"];

fn path_exists(path: &Path) -> bool {
    fs::metadata(path).is_ok()
}

fn main() {
    let use_static_lib = env::var_os("LIBCLANG_STATIC").is_some() || cfg!(feature = "static");

    let possible_clang_dirs = if let Ok(dir) = env::var("LIBCLANG_PATH") {
        vec![dir]
    } else if cfg!(any(target_os = "linux", target_os = "freebsd")) {
        LINUX_CLANG_DIRS.iter().map(ToString::to_string).collect()
    } else if cfg!(target_os = "macos") {
        MAC_CLANG_DIR.iter().map(ToString::to_string).collect()
    } else if cfg!(target_os = "windows") {
        WIN_CLANG_DIRS.iter().map(ToString::to_string).collect()
    } else {
        panic!("Platform not supported");
    };

    let clang_lib = if cfg!(target_os = "windows") {
        format!("libclang{}", env::consts::DLL_SUFFIX)
    } else {
        format!("{}clang{}", env::consts::DLL_PREFIX, env::consts::DLL_SUFFIX)
    };

    //may contain path to libclang detected via ldconfig
    let mut libclang_path_string = String::new();

    let mut maybe_clang_dir = possible_clang_dirs.iter().filter_map(|candidate_dir| {
        let clang_dir = Path::new(candidate_dir);
        let clang_path = clang_dir.join(clang_lib.clone());
        if path_exists(&*clang_path) {
            Some(clang_dir)
        } else {
            None
        }
    }).next();

    if maybe_clang_dir.is_none() && cfg!(target_os = "linux") {
        //try to find via lddconfig
        //may return line, like
        //libclang.so.3.7 (libc6,x86-64) => /usr/lib64/libclang.so.3.7
        let lddresult = Command::new("/sbin/ldconfig")
            .arg("-p")
            .output();
        if lddresult.is_ok() {
            let lddresult = lddresult.unwrap();
            let ldd_config_output = String::from_utf8_lossy(&lddresult.stdout).to_string();
            for line in ldd_config_output.lines() {
                let line_trim = line.trim();
                if line_trim.starts_with(&*clang_lib) {
                    let last_word = line_trim.rsplit(" ").next();
                    if let Some(last_word) = last_word {
                        let libclang_path = Path::new(last_word);
                        if path_exists(&libclang_path) {
                            libclang_path_string = last_word.to_owned();
                            maybe_clang_dir = Path::new(&libclang_path_string).parent();
                        }
                    }
                    break;
                }
            }
        }
    }

    macro_rules! qw {
        ($($i:ident)*) => (vec!($(stringify!($i)),*));
    }

    if let Some(clang_dir) = maybe_clang_dir {
        if use_static_lib {
            let libs = qw![
                LLVMAnalysis
                LLVMBitReader
                LLVMCore
                LLVMLTO
                LLVMLinker
                LLVMMC
                LLVMMCParser
                LLVMObjCARCOpts
                LLVMObject
                LLVMOption
                LLVMScalarOpts
                LLVMSupport
                LLVMTarget
                LLVMTransformUtils
                LLVMVectorize
                LLVMipa
                LLVMipo
                clang
                clangARCMigrate
                clangAST
                clangASTMatchers
                clangAnalysis
                clangBasic
                clangDriver
                clangEdit
                clangFormat
                clangFrontend
                clangIndex
                clangLex
                clangParse
                clangRewrite
                clangRewriteFrontend
                clangSema
                clangSerialization
                clangStaticAnalyzerCheckers
                clangStaticAnalyzerCore
                clangStaticAnalyzerFrontend
                clangTooling
            ];


            print!("cargo:rustc-flags=");
            for lib in libs {
                print!("-l static={} ", lib);
            }
            println!("-L {} -l ncursesw -l z -l stdc++", clang_dir.to_str().unwrap());
        } else{
            println!("cargo:rustc-link-search={}", clang_dir.to_str().unwrap());
            if !libclang_path_string.is_empty() {
                let libclang_path = Path::new(&libclang_path_string);
                println!("cargo:rustc-link-lib=dylib=:{}", libclang_path.file_name().unwrap().to_str().unwrap());
            } else {
                println!("cargo:rustc-link-lib=dylib=clang");
            }
        }
        println!("cargo:rerun-if-changed=");
    } else {
        panic!("Unable to find {}", clang_lib);
    }
}
