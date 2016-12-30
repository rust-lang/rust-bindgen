//! Intermediate representation for the physical layout of some type.

use std::cmp;
use super::context::BindgenContext;
use super::derive::{CanDeriveCopy, CanDeriveDebug};
use super::ty::RUST_DERIVE_IN_ARRAY_LIMIT;

/// A type that represents the struct layout of a type.
#[derive(Debug, Clone, Copy)]
pub struct Layout {
    /// The size (in bytes) of this layout.
    pub size: usize,
    /// The alignment (in bytes) of this layout.
    pub align: usize,
    /// Whether this layout's members are packed or not.
    pub packed: bool,
}

impl Layout {
    /// Construct a new `Layout` with the given `size` and `align`. It is not
    /// packed.
    pub fn new(size: usize, align: usize) -> Self {
        Layout {
            size: size,
            align: align,
            packed: false,
        }
    }

    /// Is this a zero-sized layout?
    pub fn is_zero(&self) -> bool {
        self.size == 0 && self.align == 0
    }

    /// Construct a zero-sized layout.
    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    /// Get this layout as an opaque type.
    pub fn opaque(&self) -> Opaque {
        Opaque(*self)
    }
}

/// When we are treating a type as opaque, it is just a blob with a `Layout`.
pub struct Opaque(pub Layout);

impl CanDeriveDebug for Opaque {
    type Extra = ();

    fn can_derive_debug(&self, _: &BindgenContext, _: ()) -> bool {
        let size_divisor = cmp::max(1, self.0.align);
        self.0.size / size_divisor <= RUST_DERIVE_IN_ARRAY_LIMIT
    }
}

impl<'a> CanDeriveCopy<'a> for Opaque {
    type Extra = ();

    fn can_derive_copy(&self, _: &BindgenContext, _: ()) -> bool {
        let size_divisor = cmp::max(1, self.0.align);
        self.0.size / size_divisor <= RUST_DERIVE_IN_ARRAY_LIMIT
    }

    fn can_derive_copy_in_array(&self, ctx: &BindgenContext, _: ()) -> bool {
        self.can_derive_copy(ctx, ())
    }
}
