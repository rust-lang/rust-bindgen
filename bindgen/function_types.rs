//! A public API for defining C function types.

use crate::ir::ty::TypeKind as InternalTypeKind;

/// A C value type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKind {
    /// Represents the C `void` type.
    Void,
    /// Represents the C `bool` type.
    Bool,
    /// Represents the C `char` type.
    Char,
    /// Represents the C `int` type.
    Int,
    /// Represents the C `long` type.
    Long,
    /// Represents the C `uint8_t` type.
    U8,
    /// Represents the C `uint16_t` type.
    U16,
    /// Represents the C `uint32_t` type.
    U32,
    /// Represents the C `uint64_t` type.
    U64,
    /// Represents the C `uint128_t` type.
    U128,
    /// Represents the C `int8_t` type.
    I8,
    /// Represents the C `int16_t` type.
    I16,
    /// Represents the C `int32_t` type.
    I32,
    /// Represents the C `int64_t` type.
    I64,
    /// Represents the C `int128_t` type.
    I128,
    /// Represents the C `float` type.
    F32,
    /// Represents the C `double` type.
    F64,
}

impl From<TypeKind> for InternalTypeKind {
    fn from(val: TypeKind) -> Self {
        match val {
            TypeKind::Void => InternalTypeKind::Void,
            TypeKind::Bool => {
                InternalTypeKind::Int(crate::callbacks::IntKind::Bool)
            }
            TypeKind::Char => {
                InternalTypeKind::Int(crate::callbacks::IntKind::Char {
                    is_signed: false,
                })
            }
            TypeKind::Int => {
                InternalTypeKind::Int(crate::callbacks::IntKind::Int)
            }
            TypeKind::Long => {
                InternalTypeKind::Int(crate::callbacks::IntKind::Long)
            }
            TypeKind::U8 => {
                InternalTypeKind::Int(crate::callbacks::IntKind::U8)
            }
            TypeKind::U16 => {
                InternalTypeKind::Int(crate::callbacks::IntKind::U16)
            }
            TypeKind::U32 => {
                InternalTypeKind::Int(crate::callbacks::IntKind::U32)
            }
            TypeKind::U64 => {
                InternalTypeKind::Int(crate::callbacks::IntKind::U64)
            }
            TypeKind::U128 => {
                InternalTypeKind::Int(crate::callbacks::IntKind::U128)
            }
            TypeKind::I8 => {
                InternalTypeKind::Int(crate::callbacks::IntKind::I8)
            }
            TypeKind::I16 => {
                InternalTypeKind::Int(crate::callbacks::IntKind::I16)
            }
            TypeKind::I32 => {
                InternalTypeKind::Int(crate::callbacks::IntKind::I32)
            }
            TypeKind::I64 => {
                InternalTypeKind::Int(crate::callbacks::IntKind::I64)
            }
            TypeKind::I128 => {
                InternalTypeKind::Int(crate::callbacks::IntKind::I128)
            }
            TypeKind::F32 => {
                InternalTypeKind::Float(crate::ir::ty::FloatKind::Float)
            }
            TypeKind::F64 => {
                InternalTypeKind::Float(crate::ir::ty::FloatKind::Double)
            }
        }
    }
}

/// Something that represents a C value type.
pub trait CType {
    /// Get the [TypeKind] that this C type represents.
    fn as_type() -> TypeKind;
}

// Allow `()` to be used to indicate `void`.
impl CType for () {
    fn as_type() -> TypeKind {
        TypeKind::Void
    }
}

impl CType for bool {
    fn as_type() -> TypeKind {
        TypeKind::Bool
    }
}

impl CType for char {
    fn as_type() -> TypeKind {
        TypeKind::Char
    }
}

impl CType for u8 {
    fn as_type() -> TypeKind {
        TypeKind::U8
    }
}

impl CType for u16 {
    fn as_type() -> TypeKind {
        TypeKind::U16
    }
}

impl CType for u32 {
    fn as_type() -> TypeKind {
        TypeKind::U32
    }
}

impl CType for u64 {
    fn as_type() -> TypeKind {
        TypeKind::U64
    }
}

impl CType for u128 {
    fn as_type() -> TypeKind {
        TypeKind::U128
    }
}

impl CType for i8 {
    fn as_type() -> TypeKind {
        TypeKind::I8
    }
}

impl CType for i16 {
    fn as_type() -> TypeKind {
        TypeKind::I16
    }
}

impl CType for i32 {
    fn as_type() -> TypeKind {
        TypeKind::I32
    }
}

impl CType for i64 {
    fn as_type() -> TypeKind {
        TypeKind::I64
    }
}

impl CType for i128 {
    fn as_type() -> TypeKind {
        TypeKind::I128
    }
}

impl CType for f32 {
    fn as_type() -> TypeKind {
        TypeKind::F32
    }
}

impl CType for f64 {
    fn as_type() -> TypeKind {
        TypeKind::F64
    }
}

/// Something that can be used to indicate the type of arguments for a [`FunctionType`].
pub trait ArgumentType {
    /// The list of argument types that this [`ArgumentType`] represents.
    fn as_types() -> Vec<TypeKind>;
}

impl<T: CType> ArgumentType for T {
    fn as_types() -> Vec<TypeKind> {
        vec![T::as_type()]
    }
}

impl<T1: CType> ArgumentType for (T1,) {
    fn as_types() -> Vec<TypeKind> {
        vec![T1::as_type()]
    }
}

impl<T1: CType, T2: CType> ArgumentType for (T1, T2) {
    fn as_types() -> Vec<TypeKind> {
        vec![T1::as_type(), T2::as_type()]
    }
}

impl<T1: CType, T2: CType, T3: CType> ArgumentType for (T1, T2, T3) {
    fn as_types() -> Vec<TypeKind> {
        vec![T1::as_type(), T2::as_type(), T3::as_type()]
    }
}

impl<T1: CType, T2: CType, T3: CType, T4: CType> ArgumentType
    for (T1, T2, T3, T4)
{
    fn as_types() -> Vec<TypeKind> {
        vec![T1::as_type(), T2::as_type(), T3::as_type(), T4::as_type()]
    }
}

impl<T1: CType, T2: CType, T3: CType, T4: CType, T5: CType> ArgumentType
    for (T1, T2, T3, T4, T5)
{
    fn as_types() -> Vec<TypeKind> {
        vec![
            T1::as_type(),
            T2::as_type(),
            T3::as_type(),
            T4::as_type(),
            T5::as_type(),
        ]
    }
}

impl<T1: CType, T2: CType, T3: CType, T4: CType, T5: CType, T6: CType>
    ArgumentType for (T1, T2, T3, T4, T5, T6)
{
    fn as_types() -> Vec<TypeKind> {
        vec![
            T1::as_type(),
            T2::as_type(),
            T3::as_type(),
            T4::as_type(),
            T5::as_type(),
            T6::as_type(),
        ]
    }
}

impl<
        T1: CType,
        T2: CType,
        T3: CType,
        T4: CType,
        T5: CType,
        T6: CType,
        T7: CType,
    > ArgumentType for (T1, T2, T3, T4, T5, T6, T7)
{
    fn as_types() -> Vec<TypeKind> {
        vec![
            T1::as_type(),
            T2::as_type(),
            T3::as_type(),
            T4::as_type(),
            T5::as_type(),
            T6::as_type(),
            T7::as_type(),
        ]
    }
}

impl<
        T1: CType,
        T2: CType,
        T3: CType,
        T4: CType,
        T5: CType,
        T6: CType,
        T7: CType,
        T8: CType,
    > ArgumentType for (T1, T2, T3, T4, T5, T6, T7, T8)
{
    fn as_types() -> Vec<TypeKind> {
        vec![
            T1::as_type(),
            T2::as_type(),
            T3::as_type(),
            T4::as_type(),
            T5::as_type(),
            T6::as_type(),
            T7::as_type(),
            T8::as_type(),
        ]
    }
}

impl<
        T1: CType,
        T2: CType,
        T3: CType,
        T4: CType,
        T5: CType,
        T6: CType,
        T7: CType,
        T8: CType,
        T9: CType,
    > ArgumentType for (T1, T2, T3, T4, T5, T6, T7, T8, T9)
{
    fn as_types() -> Vec<TypeKind> {
        vec![
            T1::as_type(),
            T2::as_type(),
            T3::as_type(),
            T4::as_type(),
            T5::as_type(),
            T6::as_type(),
            T7::as_type(),
            T8::as_type(),
            T9::as_type(),
        ]
    }
}

impl<
        T1: CType,
        T2: CType,
        T3: CType,
        T4: CType,
        T5: CType,
        T6: CType,
        T7: CType,
        T8: CType,
        T9: CType,
        T10: CType,
    > ArgumentType for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{
    fn as_types() -> Vec<TypeKind> {
        vec![
            T1::as_type(),
            T2::as_type(),
            T3::as_type(),
            T4::as_type(),
            T5::as_type(),
            T6::as_type(),
            T7::as_type(),
            T8::as_type(),
            T9::as_type(),
            T10::as_type(),
        ]
    }
}

impl<
        T1: CType,
        T2: CType,
        T3: CType,
        T4: CType,
        T5: CType,
        T6: CType,
        T7: CType,
        T8: CType,
        T9: CType,
        T10: CType,
        T11: CType,
    > ArgumentType for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
    fn as_types() -> Vec<TypeKind> {
        vec![
            T1::as_type(),
            T2::as_type(),
            T3::as_type(),
            T4::as_type(),
            T5::as_type(),
            T6::as_type(),
            T7::as_type(),
            T8::as_type(),
            T9::as_type(),
            T10::as_type(),
            T11::as_type(),
        ]
    }
}

impl<
        T1: CType,
        T2: CType,
        T3: CType,
        T4: CType,
        T5: CType,
        T6: CType,
        T7: CType,
        T8: CType,
        T9: CType,
        T10: CType,
        T11: CType,
        T12: CType,
    > ArgumentType for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
{
    fn as_types() -> Vec<TypeKind> {
        vec![
            T1::as_type(),
            T2::as_type(),
            T3::as_type(),
            T4::as_type(),
            T5::as_type(),
            T6::as_type(),
            T7::as_type(),
            T8::as_type(),
            T9::as_type(),
            T10::as_type(),
            T11::as_type(),
            T12::as_type(),
        ]
    }
}

/// The type of a C function.
#[derive(Debug, Clone)]
pub struct FunctionType {
    /// The return type of the function.
    pub(crate) return_type: TypeKind,

    /// The type of the arguments.
    pub(crate) argument_types: Vec<TypeKind>,
}

impl FunctionType {
    /// Create a new function type with the given `ReturnType` and `ArgumentTypes`.
    pub fn new<ReturnType: CType, ArgumentTypes: ArgumentType>() -> Self {
        Self::from(ReturnType::as_type(), ArgumentTypes::as_types())
    }

    /// Create a new function type from the given `return_type` and `argument_types`.
    /// If `argument_types` is a vector with a single [`TypeKind::Void`] item,
    /// the function type will have an empty list of arguments, following the
    /// C standard to describe functions which don't take any arguments: `function(void)`.
    pub fn from(return_type: TypeKind, argument_types: Vec<TypeKind>) -> Self {
        let is_void_only_arguments =
            argument_types.len() == 1 && argument_types[0] == TypeKind::Void;
        Self {
            return_type,
            argument_types: if is_void_only_arguments {
                Vec::new()
            } else {
                argument_types
            },
        }
    }
}
