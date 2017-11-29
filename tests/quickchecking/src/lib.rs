//! A library to generate __fuzzed__ C headers for use with `quickcheck`
//!
//! ## Example
//!
//! ```rust
//! extern crate quickcheck;
//! extern crate quickchecking;
//! extern crate rand;
//! use quickcheck::{Arbitrary, Gen, StdGen};
//! use quickchecking::fuzzers;
//! use rand::thread_rng;
//!
//! fn main() {
//!     let generate_range: usize = 10; // Determines things like the length of
//!                                     // arbitrary vectors generated.
//!     let header = fuzzers::HeaderC::arbitrary(
//!        &mut StdGen::new(thread_rng(), generate_range));
//!     println!("{}", header);
//! }
//! ```
//!
#![deny(missing_docs)]
extern crate quickcheck;
extern crate rand;
extern crate tempdir;

/// Contains definitions of and impls for types used to fuzz C declarations.
pub mod fuzzers;
