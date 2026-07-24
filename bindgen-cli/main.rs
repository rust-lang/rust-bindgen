use std::env;

use bindgen::builder_from_flags;

#[cfg(feature = "logging")]
fn clang_version_check() {
    let version = bindgen::clang_version();
    log::info!(
        "Clang Version: {}, parsed: {:?}",
        version.full,
        version.parsed
    );
}

pub fn main() {
    #[cfg(feature = "logging")]
    env_logger::init();

    match builder_from_flags(env::args()) {
        Ok((builder, mut output, verbose)) => {
            #[cfg(feature = "logging")]
            clang_version_check();

            std::panic::set_hook(Box::new(move |info| {
                if verbose {
                    print_verbose_err();
                }
                eprintln!("{info}");
            }));

            let bindings = match builder.generate() {
                Ok(bindings) => bindings,
                Err(err) => {
                    eprintln!("Unable to generate bindings: {err}");
                    std::process::exit(1)
                }
            };

            let _ = std::panic::take_hook();

            bindings.write(&mut output).expect("Unable to write output");
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };
}

fn print_verbose_err() {
    eprintln!("Bindgen unexpectedly panicked");
    eprintln!(
        "This may be caused by one of the known-unsupported \
         things (https://rust-lang.github.io/rust-bindgen/cpp.html), \
         please modify the bindgen flags to work around it as \
         described in https://rust-lang.github.io/rust-bindgen/cpp.html"
    );
    eprintln!(
        "Otherwise, please file an issue at \
         https://github.com/rust-lang/rust-bindgen/issues/new"
    );
}
