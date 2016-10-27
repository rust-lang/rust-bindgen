//! Intermediate representation for the physical layout of some type.

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
}
