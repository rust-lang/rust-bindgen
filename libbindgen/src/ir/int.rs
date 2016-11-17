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

    /// A 8-bit signed integer.
    I8,

    /// A 8-bit unsigned integer.
    U8,

    /// A 16-bit signed integer.
    I16,

    /// Either a `char16_t` or a `wchar_t`.
    U16,

    /// A 32-bit signed integer.
    I32,

    /// A 32-bit unsigned integer.
    U32,

    /// A 64-bit signed integer.
    I64,

    /// A 64-bit unsigned integer.
    U64,

    /// An `int128_t`
    I128,

    /// A `uint128_t`.
    U128,

    /// A custom integer type, used to allow custom macro types depending on
    /// range.
    Custom {
        /// The name of the type, which would be used without modification.
        name: &'static str,
        /// Whether the type is signed or not.
        is_signed: bool,
    },
}

impl IntKind {
    /// Is this integral type signed?
    pub fn is_signed(&self) -> bool {
        use self::IntKind::*;
        match *self {
            Bool | UChar | UShort | UInt | ULong | ULongLong | U8 | U16 |
            U32 | U64 | U128 => false,

            Char | Short | Int | Long | LongLong | I8 | I16 | I32 | I64 |
            I128 => true,

            Custom { is_signed, .. } => is_signed,
        }
    }

    /// Whether this type's signedness matches the value.
    pub fn signedness_matches(&self, val: i64) -> bool {
        val >= 0 || self.is_signed()
    }
}
