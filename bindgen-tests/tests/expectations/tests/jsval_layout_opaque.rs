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
        let byte = unsafe {
            *(core::ptr::addr_of!((*this).storage) as *const u8)
                .offset(byte_index as isize)
        };
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
        let byte = unsafe {
            (core::ptr::addr_of_mut!((*this).storage) as *mut u8)
                .offset(byte_index as isize)
        };
        unsafe { *byte = Self::change_bit(*byte, index, val) };
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        let Self { storage } = self;
        let storage = storage.as_ref();
        unsafe { Self::get_bits(storage.as_ptr(), storage.len(), bit_offset, bit_width) }
    }
    #[inline]
    pub unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64 {
        unsafe {
            Self::get_bits(
                core::ptr::addr_of!((*this).storage).cast(),
                core::mem::size_of::<Storage>(),
                bit_offset,
                bit_width,
            )
        }
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        let Self { storage } = self;
        let storage = storage.as_mut();
        unsafe {
            Self::set_bits(
                storage.as_mut_ptr(),
                storage.len(),
                bit_offset,
                bit_width,
                val,
            );
        }
    }
    #[inline]
    pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) {
        unsafe {
            Self::set_bits(
                core::ptr::addr_of_mut!((*this).storage).cast(),
                core::mem::size_of::<Storage>(),
                bit_offset,
                bit_width,
                val,
            );
        }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    const fn field_layout(
        storage_len: usize,
        bit_offset: usize,
        bit_width: u8,
    ) -> (usize, usize, usize) {
        debug_assert!(bit_width <= 64);
        let start_byte = bit_offset / 8;
        let bit_shift = bit_offset % 8;
        debug_assert!(start_byte < storage_len);
        let bytes_needed = (bit_width as usize + bit_shift + 7) / 8;
        assert!(start_byte + bytes_needed <= storage_len);
        (start_byte, bit_shift, bytes_needed)
    }
    #[inline]
    const unsafe fn get_bits(
        storage: *const u8,
        storage_len: usize,
        bit_offset: usize,
        bit_width: u8,
    ) -> u64 {
        let (start_byte, bit_shift, bytes_needed) = Self::field_layout(
            storage_len,
            bit_offset,
            bit_width,
        );
        if bit_width == 0 {
            return 0;
        }
        let mut val = 0u64;
        let mut i = 0;
        while i < bytes_needed && i < 8 {
            let byte = unsafe { *storage.add(start_byte + i) };
            let byte = if cfg!(target_endian = "big") {
                byte.reverse_bits()
            } else {
                byte
            };
            val |= (byte as u64) << (i * 8);
            i += 1;
        }
        val >>= bit_shift;
        if bytes_needed == 9 {
            let byte = unsafe { *storage.add(start_byte + 8) };
            let byte = if cfg!(target_endian = "big") {
                byte.reverse_bits()
            } else {
                byte
            };
            val |= (byte as u64) << (64 - bit_shift);
        }
        if bit_width < 64 {
            val &= (1u64 << bit_width) - 1;
        }
        if cfg!(target_endian = "big") {
            val = val.reverse_bits() >> (64 - bit_width as usize);
        }
        val
    }
    #[inline]
    unsafe fn set_bits(
        storage: *mut u8,
        storage_len: usize,
        bit_offset: usize,
        bit_width: u8,
        mut val: u64,
    ) {
        let (start_byte, bit_shift, bytes_needed) = Self::field_layout(
            storage_len,
            bit_offset,
            bit_width,
        );
        if bit_width == 0 {
            return;
        }
        if bit_width == 64 && bit_shift == 0 {
            unsafe { storage.add(start_byte).cast::<u64>().write_unaligned(val) };
            return;
        }
        let field_mask = if bit_width == 64 { !0u64 } else { (1u64 << bit_width) - 1 };
        val &= field_mask;
        if cfg!(target_endian = "big") {
            val = val.reverse_bits() >> (64 - bit_width as usize);
        }
        if bytes_needed == 9 {
            let shift = 64 - bit_shift;
            unsafe {
                Self::set_byte(
                    storage.add(start_byte + 8),
                    (val >> shift) as u8,
                    (field_mask >> shift) as u8,
                );
            }
        }
        let val = val << bit_shift;
        let field_mask = field_mask << bit_shift;
        for i in 0..bytes_needed.min(8) {
            unsafe {
                Self::set_byte(
                    storage.add(start_byte + i),
                    (val >> (i * 8)) as u8,
                    (field_mask >> (i * 8)) as u8,
                );
            }
        }
    }
    #[inline]
    unsafe fn set_byte(byte: *mut u8, val: u8, mask: u8) {
        let (val, mask) = if cfg!(target_endian = "big") {
            (val.reverse_bits(), mask.reverse_bits())
        } else {
            (val, mask)
        };
        unsafe { *byte = (*byte & !mask) | (val & mask) };
    }
}
/// Const-generic methods for bitfield access when offset and width are known
/// at compile time. Inlining exposes these constants to the shared implementation.
impl<const N: usize> __BindgenBitfieldUnit<[u8; N]> {
    /// Get a field using const generics for compile-time optimization.
    #[inline]
    pub const fn get_const<const BIT_OFFSET: usize, const BIT_WIDTH: u8>(&self) -> u64 {
        let Self { storage } = self;
        unsafe { Self::get_bits(storage.as_ptr(), N, BIT_OFFSET, BIT_WIDTH) }
    }
    /// Set a field using const generics for compile-time optimization.
    #[inline]
    pub fn set_const<const BIT_OFFSET: usize, const BIT_WIDTH: u8>(&mut self, val: u64) {
        self.set(BIT_OFFSET, BIT_WIDTH, val);
    }
    /// Raw pointer get using const generics for compile-time optimization.
    #[inline]
    pub const unsafe fn raw_get_const<const BIT_OFFSET: usize, const BIT_WIDTH: u8>(
        this: *const Self,
    ) -> u64 {
        unsafe {
            Self::get_bits(
                core::ptr::addr_of!((*this).storage).cast(),
                N,
                BIT_OFFSET,
                BIT_WIDTH,
            )
        }
    }
    /// Raw pointer set using const generics for compile-time optimization.
    #[inline]
    pub unsafe fn raw_set_const<const BIT_OFFSET: usize, const BIT_WIDTH: u8>(
        this: *mut Self,
        val: u64,
    ) {
        unsafe { Self::raw_set(this, BIT_OFFSET, BIT_WIDTH, val) };
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
    pub _bindgen_align: [u64; 0],
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
    #[allow(unnecessary_transmutes)]
    pub fn payload47(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<0usize, 47u8>() as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_payload47(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<0usize, 47u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn payload47_raw(this: *const Self) -> u64 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 8usize],
                >>::raw_get_const::<
                    0usize,
                    47u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u64,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_payload47_raw(this: *mut Self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 8usize],
            >>::raw_set_const::<
                0usize,
                47u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn tag(&self) -> JSValueTag {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<47usize, 17u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_tag(&mut self, val: JSValueTag) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<47usize, 17u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn tag_raw(this: *const Self) -> JSValueTag {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 8usize],
                >>::raw_get_const::<
                    47usize,
                    17u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_tag_raw(this: *mut Self, val: JSValueTag) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 8usize],
            >>::raw_set_const::<
                47usize,
                17u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn new_bitfield_1(
        payload47: u64,
        tag: JSValueTag,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit
            .set_const::<
                0usize,
                47u8,
            >({
                let payload47: u64 = unsafe { ::std::mem::transmute(payload47) };
                payload47 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                47usize,
                17u8,
            >({
                let tag: u32 = unsafe { ::std::mem::transmute(tag) };
                tag as u64
            });
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
