//! Intermediate representation for integral types.

/// Which integral type are we dealing with?
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IntKind {
    /// A `bool`.
    Bool,

    /// A `signed char`.
    SChar,

    /// An `unsigned char`.
    UChar,

    /// A `wchar_t`.
    WChar,

    /// A platform-dependent `char` type, with the signedness support.
    Char {
        /// Whether the char is signed for the target platform.
        is_signed: bool,
    },

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

    /// A 16-bit integer, used only for enum size representation.
    U16,

    /// The C++ type `char16_t`, which is its own type (unlike in C).
    Char16,

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
    pub(crate) fn is_signed(&self) -> bool {
        use self::IntKind::*;
        match *self {
            // TODO(emilio): wchar_t can in theory be signed, but we have no way
            // to know whether it is or not right now (unlike char, there's no
            // WChar_S / WChar_U).
            Bool | UChar | UShort | UInt | ULong | ULongLong | U8 | U16 |
            Char16 | WChar | U32 | U64 | U128 => false,

            SChar | Short | Int | Long | LongLong | I8 | I16 | I32 | I64 |
            I128 => true,

            Char { is_signed } | Custom { is_signed, .. } => is_signed,
        }
    }

    /// If this type has a known size, return it (in bytes). This is to
    /// alleviate libclang sometimes not giving us a layout (like in the case
    /// when an enum is defined inside a class with template parameters).
    pub(crate) fn known_size(&self) -> Option<usize> {
        use self::IntKind::*;
        Some(match *self {
            Bool | UChar | SChar | U8 | I8 | Char { .. } => 1,
            U16 | I16 | Char16 => 2,
            U32 | I32 => 4,
            U64 | I64 => 8,
            I128 | U128 => 16,
            _ => return None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::IntKind;

    #[test]
    fn signedness() {
        let cases: &[(IntKind, bool)] = &[
            (IntKind::Bool, false),
            (IntKind::SChar, true),
            (IntKind::UChar, false),
            (IntKind::WChar, false),
            (IntKind::Char { is_signed: true }, true),
            (IntKind::Char { is_signed: false }, false),
            (IntKind::Short, true),
            (IntKind::UShort, false),
            (IntKind::Int, true),
            (IntKind::UInt, false),
            (IntKind::Long, true),
            (IntKind::ULong, false),
            (IntKind::LongLong, true),
            (IntKind::ULongLong, false),
            (IntKind::I8, true),
            (IntKind::U8, false),
            (IntKind::I16, true),
            (IntKind::U16, false),
            (IntKind::Char16, false),
            (IntKind::I32, true),
            (IntKind::U32, false),
            (IntKind::I64, true),
            (IntKind::U64, false),
            (IntKind::I128, true),
            (IntKind::U128, false),
            (IntKind::Custom { name: "x", is_signed: true }, true),
            (IntKind::Custom { name: "x", is_signed: false }, false),
        ];

        for &(kind, expected) in cases {
            assert_eq!(
                kind.is_signed(),
                expected,
                "unexpected signedness for {kind:?}"
            );
        }
    }

    #[test]
    fn known_size() {
        let cases: &[(IntKind, Option<usize>)] = &[
            (IntKind::Bool, Some(1)),
            (IntKind::SChar, Some(1)),
            (IntKind::UChar, Some(1)),
            (IntKind::Char { is_signed: true }, Some(1)),
            (IntKind::Char { is_signed: false }, Some(1)),
            (IntKind::I8, Some(1)),
            (IntKind::U8, Some(1)),
            (IntKind::I16, Some(2)),
            (IntKind::U16, Some(2)),
            (IntKind::Char16, Some(2)),
            (IntKind::I32, Some(4)),
            (IntKind::U32, Some(4)),
            (IntKind::I64, Some(8)),
            (IntKind::U64, Some(8)),
            (IntKind::I128, Some(16)),
            (IntKind::U128, Some(16)),
            (IntKind::WChar, None),
            (IntKind::Short, None),
            (IntKind::UShort, None),
            (IntKind::Int, None),
            (IntKind::UInt, None),
            (IntKind::Long, None),
            (IntKind::ULong, None),
            (IntKind::LongLong, None),
            (IntKind::ULongLong, None),
            (IntKind::Custom { name: "x", is_signed: true }, None),
        ];

        for &(kind, expected) in cases {
            assert_eq!(
                kind.known_size(),
                expected,
                "unexpected known_size for {kind:?}"
            );
        }
    }
}
