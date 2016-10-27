//! Intermediate representation for integral types.

/// Which integral type are we dealing with?
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IntKind {
    /// A `bool`.
    Bool,

    /// A `char`.
    Char,

    /// An `unsigned char`.
    UChar,

    /// A `short`.
    Short,

    /// An `unsigned short`.
    UShort,

    /// An `int`.
    Int,

    /// An `unsigned int`.
    UInt,

    /// A `long`.
    Long,

    /// An `unsigned long`.
    ULong,

    /// A `long long`.
    LongLong,

    /// An `unsigned long long`.
    ULongLong,

    /// Either a `char16_t` or a `wchar_t`.
    U16,

    /// A `char32_t`.
    U32,

    /// An `int128_t`
    I128,

    /// A `uint128_t`.
    U128,

    // Though now we're at it we could add equivalents for the rust types...
}

impl IntKind {
    /// Is this integral type signed?
    pub fn is_signed(&self) -> bool {
        use self::IntKind::*;
        match *self {
            Bool | UChar | UShort |
            UInt | ULong | ULongLong | U16 | U32 | U128 => false,

            Char | Short | Int |
            Long | LongLong | I128 => true,
        }
    }
}
