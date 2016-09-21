
/// A type that represents the struct layout of a type.
#[derive(Debug, Clone, Copy)]
pub struct Layout {
    pub size: usize,
    pub align: usize,
    pub packed: bool,
}

impl Layout {
    pub fn new(size: usize, align: usize) -> Self {
        Layout {
            size: size,
            align: align,
            packed: false,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.size == 0 && self.align == 0
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }
}
