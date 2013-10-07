#[crate_type = "bin"];
#[feature(globs)];

extern mod syntax;

pub mod types;
pub mod clangll;
pub mod clang;
pub mod gen;
pub mod main;
