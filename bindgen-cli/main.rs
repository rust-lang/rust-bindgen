extern crate bindgen;
extern crate clap;
#[cfg(feature = "logging")]
extern crate env_logger;
#[cfg(feature = "logging")]
extern crate log;

use std::env;

mod options;
use crate::options::builder_from_flags;

#[cfg(feature = "logging")]
fn clang_version_check() {
    let version = bindgen::clang_version();
    let expected_version = if cfg!(feature = "testing_only_libclang_9") {
        Some((9, 0))
    } else if cfg!(feature = "testing_only_libclang_5") {
        Some((5, 0))
    } else {
        None
    };

    log::info!(
        "Clang Version: {}, parsed: {:?}",
        version.full,
        version.parsed
    );

    if expected_version.is_some() {
        // assert_eq!(version.parsed, version.parsed);
    }
}

pub fn main() {
    #[cfg(feature = "logging")]
    env_logger::init();

    match builder_from_flags(env::args()) {
        Ok((builder, output, verbose)) => {
            #[cfg(feature = "logging")]
            clang_version_check();

            std::panic::set_hook(Box::new(move |info| {
                if verbose {
                    print_verbose_err()
                }
                println!("{}", info);
            }));

            let bindings =
                builder.generate().expect("Unable to generate bindings");

            let _ = std::panic::take_hook();

            bindings.write(output).expect("Unable to write output");
        }
        Err(error) => {
            println!("{}", error);
            std::process::exit(1);
        }
    };
}

fn print_verbose_err() {
    println!("Bindgen unexpectedly panicked");
    println!(
        "This may be caused by one of the known-unsupported \
         things (https://rust-lang.github.io/rust-bindgen/cpp.html), \
         please modify the bindgen flags to work around it as \
         described in https://rust-lang.github.io/rust-bindgen/cpp.html"
    );
    println!(
        "Otherwise, please file an issue at \
         https://github.com/rust-lang/rust-bindgen/issues/new"
    );
}
