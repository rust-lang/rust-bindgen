#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    fn extract_bit(byte: u8, index: usize) -> bool {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        Self::extract_bit(byte, index)
    }
    #[inline]
    pub unsafe fn raw_get_bit(this: *const Self, index: usize) -> bool {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = *(core::ptr::addr_of!((*this).storage) as *const u8)
            .offset(byte_index as isize);
        Self::extract_bit(byte, index)
    }
    #[inline]
    fn change_bit(byte: u8, index: usize, val: bool) -> u8 {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val { byte | mask } else { byte & !mask }
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub unsafe fn raw_set_bit(this: *mut Self, index: usize, val: bool) {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = (core::ptr::addr_of_mut!((*this).storage) as *mut u8)
            .offset(byte_index as isize);
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len(),
        );
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>(),
        );
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if Self::raw_get_bit(this, i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len(),
        );
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
    #[inline]
    pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>(),
        );
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            Self::raw_set_bit(this, index + bit_offset, val_bit_is_set);
        }
    }
}
pub const JSVAL_TAG_SHIFT: u32 = 47;
pub const JSVAL_PAYLOAD_MASK: u64 = 140737488355327;
pub const JSVAL_TAG_MASK: i64 = -140737488355328;
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum JSValueType {
    JSVAL_TYPE_DOUBLE = 0,
    JSVAL_TYPE_INT32 = 1,
    JSVAL_TYPE_UNDEFINED = 2,
    JSVAL_TYPE_BOOLEAN = 3,
    JSVAL_TYPE_MAGIC = 4,
    JSVAL_TYPE_STRING = 5,
    JSVAL_TYPE_SYMBOL = 6,
    JSVAL_TYPE_NULL = 7,
    JSVAL_TYPE_OBJECT = 8,
    JSVAL_TYPE_UNKNOWN = 32,
    JSVAL_TYPE_MISSING = 33,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum JSValueTag {
    JSVAL_TAG_MAX_DOUBLE = 131056,
    JSVAL_TAG_INT32 = 131057,
    JSVAL_TAG_UNDEFINED = 131058,
    JSVAL_TAG_STRING = 131061,
    JSVAL_TAG_SYMBOL = 131062,
    JSVAL_TAG_BOOLEAN = 131059,
    JSVAL_TAG_MAGIC = 131060,
    JSVAL_TAG_NULL = 131063,
    JSVAL_TAG_OBJECT = 131064,
}
#[repr(u64)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum JSValueShiftedTag {
    JSVAL_SHIFTED_TAG_MAX_DOUBLE = 18444492278190833663,
    JSVAL_SHIFTED_TAG_INT32 = 18444633011384221696,
    JSVAL_SHIFTED_TAG_UNDEFINED = 18444773748872577024,
    JSVAL_SHIFTED_TAG_STRING = 18445195961337643008,
    JSVAL_SHIFTED_TAG_SYMBOL = 18445336698825998336,
    JSVAL_SHIFTED_TAG_BOOLEAN = 18444914486360932352,
    JSVAL_SHIFTED_TAG_MAGIC = 18445055223849287680,
    JSVAL_SHIFTED_TAG_NULL = 18445477436314353664,
    JSVAL_SHIFTED_TAG_OBJECT = 18445618173802708992,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum JSWhyMagic {
    /// a hole in a native object's elements
    JS_ELEMENTS_HOLE = 0,
    /// there is not a pending iterator value
    JS_NO_ITER_VALUE = 1,
    /// exception value thrown when closing a generator
    JS_GENERATOR_CLOSING = 2,
    /// compiler sentinel value
    JS_NO_CONSTANT = 3,
    /// used in debug builds to catch tracing errors
    JS_THIS_POISON = 4,
    /// used in debug builds to catch tracing errors
    JS_ARG_POISON = 5,
    /// an empty subnode in the AST serializer
    JS_SERIALIZE_NO_NODE = 6,
    /// lazy arguments value on the stack
    JS_LAZY_ARGUMENTS = 7,
    /// optimized-away 'arguments' value
    JS_OPTIMIZED_ARGUMENTS = 8,
    /// magic value passed to natives to indicate construction
    JS_IS_CONSTRUCTING = 9,
    /// arguments.callee has been overwritten
    JS_OVERWRITTEN_CALLEE = 10,
    /// value of static block object slot
    JS_BLOCK_NEEDS_CLONE = 11,
    /// see class js::HashableValue
    JS_HASH_KEY_EMPTY = 12,
    /// error while running Ion code
    JS_ION_ERROR = 13,
    /// missing recover instruction result
    JS_ION_BAILOUT = 14,
    /// optimized out slot
    JS_OPTIMIZED_OUT = 15,
    /// uninitialized lexical bindings that produce ReferenceError on touch.
    JS_UNINITIALIZED_LEXICAL = 16,
    /// for local use
    JS_GENERIC_MAGIC = 17,
    /// for local use
    JS_WHY_MAGIC_COUNT = 18,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union jsval_layout {
    pub asBits: u64,
    pub debugView: jsval_layout__bindgen_ty_1,
    pub s: jsval_layout__bindgen_ty_2,
    pub asDouble: f64,
    pub asPtr: *mut ::std::os::raw::c_void,
    pub asWord: usize,
    pub asUIntPtr: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct jsval_layout__bindgen_ty_1 {
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of jsval_layout__bindgen_ty_1",
    ][::std::mem::size_of::<jsval_layout__bindgen_ty_1>() - 8usize];
    [
        "Alignment of jsval_layout__bindgen_ty_1",
    ][::std::mem::align_of::<jsval_layout__bindgen_ty_1>() - 8usize];
};
impl Default for jsval_layout__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl jsval_layout__bindgen_ty_1 {
    #[inline]
    pub fn payload47(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 47u8) as u64) }
    }
    #[inline]
    pub fn set_payload47(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 47u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn payload47_raw(this: *const Self) -> u64 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 8usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 0usize, 47u8)
                    as u64,
            )
        }
    }
    #[inline]
    pub unsafe fn set_payload47_raw(this: *mut Self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 8usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                47u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn tag(&self) -> JSValueTag {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(47usize, 17u8) as u32) }
    }
    #[inline]
    pub fn set_tag(&mut self, val: JSValueTag) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(47usize, 17u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn tag_raw(this: *const Self) -> JSValueTag {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 8usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 47usize, 17u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_tag_raw(this: *mut Self, val: JSValueTag) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 8usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                47usize,
                17u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        payload47: u64,
        tag: JSValueTag,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                47u8,
                {
                    let payload47: u64 = unsafe { ::std::mem::transmute(payload47) };
                    payload47 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                47usize,
                17u8,
                {
                    let tag: u32 = unsafe { ::std::mem::transmute(tag) };
                    tag as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jsval_layout__bindgen_ty_2 {
    pub payload: jsval_layout__bindgen_ty_2__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union jsval_layout__bindgen_ty_2__bindgen_ty_1 {
    pub i32_: i32,
    pub u32_: u32,
    pub why: JSWhyMagic,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of jsval_layout__bindgen_ty_2__bindgen_ty_1",
    ][::std::mem::size_of::<jsval_layout__bindgen_ty_2__bindgen_ty_1>() - 4usize];
    [
        "Alignment of jsval_layout__bindgen_ty_2__bindgen_ty_1",
    ][::std::mem::align_of::<jsval_layout__bindgen_ty_2__bindgen_ty_1>() - 4usize];
    [
        "Offset of field: jsval_layout__bindgen_ty_2__bindgen_ty_1::i32_",
    ][::std::mem::offset_of!(jsval_layout__bindgen_ty_2__bindgen_ty_1, i32_) - 0usize];
    [
        "Offset of field: jsval_layout__bindgen_ty_2__bindgen_ty_1::u32_",
    ][::std::mem::offset_of!(jsval_layout__bindgen_ty_2__bindgen_ty_1, u32_) - 0usize];
    [
        "Offset of field: jsval_layout__bindgen_ty_2__bindgen_ty_1::why",
    ][::std::mem::offset_of!(jsval_layout__bindgen_ty_2__bindgen_ty_1, why) - 0usize];
};
impl Default for jsval_layout__bindgen_ty_2__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of jsval_layout__bindgen_ty_2",
    ][::std::mem::size_of::<jsval_layout__bindgen_ty_2>() - 4usize];
    [
        "Alignment of jsval_layout__bindgen_ty_2",
    ][::std::mem::align_of::<jsval_layout__bindgen_ty_2>() - 4usize];
    [
        "Offset of field: jsval_layout__bindgen_ty_2::payload",
    ][::std::mem::offset_of!(jsval_layout__bindgen_ty_2, payload) - 0usize];
};
impl Default for jsval_layout__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of jsval_layout"][::std::mem::size_of::<jsval_layout>() - 8usize];
    ["Alignment of jsval_layout"][::std::mem::align_of::<jsval_layout>() - 8usize];
    [
        "Offset of field: jsval_layout::asBits",
    ][::std::mem::offset_of!(jsval_layout, asBits) - 0usize];
    [
        "Offset of field: jsval_layout::debugView",
    ][::std::mem::offset_of!(jsval_layout, debugView) - 0usize];
    [
        "Offset of field: jsval_layout::s",
    ][::std::mem::offset_of!(jsval_layout, s) - 0usize];
    [
        "Offset of field: jsval_layout::asDouble",
    ][::std::mem::offset_of!(jsval_layout, asDouble) - 0usize];
    [
        "Offset of field: jsval_layout::asPtr",
    ][::std::mem::offset_of!(jsval_layout, asPtr) - 0usize];
    [
        "Offset of field: jsval_layout::asWord",
    ][::std::mem::offset_of!(jsval_layout, asWord) - 0usize];
    [
        "Offset of field: jsval_layout::asUIntPtr",
    ][::std::mem::offset_of!(jsval_layout, asUIntPtr) - 0usize];
};
impl Default for jsval_layout {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Value {
    pub data: jsval_layout,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Value"][::std::mem::size_of::<Value>() - 8usize];
    ["Alignment of Value"][::std::mem::align_of::<Value>() - 8usize];
    ["Offset of field: Value::data"][::std::mem::offset_of!(Value, data) - 0usize];
};
impl Default for Value {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
