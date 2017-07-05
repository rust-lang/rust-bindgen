extern crate bindgen;
#[cfg(feature="logging")]
extern crate env_logger;
#[macro_use]
#[cfg(feature="logging")]
extern crate log;
extern crate clang_sys;
extern crate clap;

use std::env;
use std::panic;

mod options;
use options::builder_from_flags;

pub fn main() {
    #[cfg(feature="logging")]
    log::set_logger(|max_log_level| {
            use env_logger::Logger;
            let env_logger = Logger::new();
            max_log_level.set(env_logger.filter());
            Box::new(env_logger)
        })
        .expect("Failed to set logger.");

    let bind_args: Vec<_> = env::args().collect();

    match builder_from_flags(bind_args.into_iter()) {
        Ok((builder, output, verbose)) => {

            let builder_result = panic::catch_unwind(|| {
                builder.generate().expect("Unable to generate bindings")
            });

            if builder_result.is_err() {
                if verbose {
                    print_verbose_err();
                }
                std::process::exit(1);
            }

            let mut bindings = builder_result.unwrap();
            bindings.write(output)
                .expect("Unable to write output");
            bindings.write_dummy_uses()
                .expect("Unable to write dummy uses to file.");
        }
        Err(error) => {
            println!("{}", error);
            std::process::exit(1);
        }
    };
}

fn print_verbose_err() {
    println!("Bindgen unexpectedly panicked");
    println!("This may be caused by one of the known-unsupported \
              things (https://github.com/servo/rust-bindgen#c), \
              please modify the bindgen flags to work around it as \
              described in https://github.com/servo/rust-bindgen#c");
    println!("Otherwise, please file an issue at \
              https://github.com/servo/rust-bindgen/issues/new");
}
