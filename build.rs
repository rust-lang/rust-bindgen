use std::os;
use std::old_io::fs::PathExtensions;

fn main() {
    let clang_dir = if let Some(dir) = os::getenv("LIBCLANG_PATH") {
        dir
    } else if cfg!(any(target_os = "linux", target_os = "freebsd")) {
        "/usr/lib".to_string()
    } else if cfg!(target_os = "macos") {
        "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib".to_string()
    } else {
        panic!("Platform not supported");
    };

    let clang_dir = Path::new(clang_dir);

    let clang_lib = os::dll_filename("clang");

    let clang_path = clang_dir.join(clang_lib.clone());
    if !clang_path.exists() {
        panic!("Unable to find {}", clang_lib);
    }

    println!("cargo:rustc-flags=-l clang -L {}", clang_dir.as_str().unwrap());
}
