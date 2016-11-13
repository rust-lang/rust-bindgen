#![crate_name = "bindgen"]
#![crate_type = "bin"]

extern crate bindgen;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate clang_sys;
extern crate clap;
extern crate rustc_serialize;

use bindgen::clang_version;
use std::env;

mod options;
use options::builder_from_flags;

pub fn main() {
    log::set_logger(|max_log_level| {
            use env_logger::Logger;
            let env_logger = Logger::new();
            max_log_level.set(env_logger.filter());
            Box::new(env_logger)
        })
        .expect("Failed to set logger.");

    let mut bind_args: Vec<_> = env::args().collect();

    let version = clang_version();
    let expected_version = if cfg!(feature = "llvm_stable") {
        (3, 8)
    } else {
        (3, 9)
    };

    info!("Clang Version: {}", version.full);

    match version.parsed {
        None => warn!("Couldn't parse libclang version"),
        Some(version) if version != expected_version => {
            error!("Using clang {:?}, expected {:?}",
                   version,
                   expected_version);
        }
        _ => {}
    }

    if let Some(clang) = clang_sys::support::Clang::find(None) {
        let has_clang_args =
            bind_args.iter().rposition(|arg| *arg == "--").is_some();
        if !has_clang_args {
            bind_args.push("--".to_owned());
        }

        // If --target is specified, assume caller knows what they're doing and
        // don't mess with
        // include paths for them
        let has_target_arg = bind_args.iter()
            .rposition(|arg| arg.starts_with("--target"))
            .is_some();
        if !has_target_arg {
            // TODO: distinguish C and C++ paths? C++'s should be enough, I
            // guess.
            for path in clang.cpp_search_paths.into_iter() {
                if let Ok(path) = path.into_os_string().into_string() {
                    bind_args.push("-isystem".to_owned());
                    bind_args.push(path);
                }
            }
        }
    }

    match builder_from_flags(bind_args.into_iter()) {
        Ok((builder, output)) => {
            let mut bindings = builder.generate()
                .expect("Unable to generate bindings");
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
