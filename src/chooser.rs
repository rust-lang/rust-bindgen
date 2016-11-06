//! A public API for more fine-grained customization of bindgen behavior.

pub use ir::int::IntKind;
use std::fmt;

/// A trait to allow configuring different kinds of types in different
/// situations.
pub trait TypeChooser: fmt::Debug {
    /// The integer kind an integer macro should have, given a name and the
    /// value of that macro, or `None` if you want the default to be chosen.
    fn int_macro(&self, _name: &str, _value: i64) -> Option<IntKind> {
        None
    }
}
