#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IntKind {
    Bool,
    Char,
    UChar,
    Short,
    UShort,
    Int,
    UInt,
    Long,
    ULong,
    LongLong,
    ULongLong,
    U16, // For Char16 and Wchar
    U32, // For Char32
    I128,
    U128,
    // Though now we're at it we could add equivalents for the rust types...
}

impl IntKind {
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
