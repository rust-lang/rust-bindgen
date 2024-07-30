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
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct redundant_packed {
    pub a: u32,
    pub b: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of redundant_packed"][::std::mem::size_of::<redundant_packed>() - 8usize];
    [
        "Alignment of redundant_packed",
    ][::std::mem::align_of::<redundant_packed>() - 8usize];
    [
        "Offset of field: redundant_packed::a",
    ][::std::mem::offset_of!(redundant_packed, a) - 0usize];
    [
        "Offset of field: redundant_packed::b",
    ][::std::mem::offset_of!(redundant_packed, b) - 4usize];
};
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct redundant_packed_bitfield {
    pub a: [u8; 3usize],
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub c: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of redundant_packed_bitfield",
    ][::std::mem::size_of::<redundant_packed_bitfield>() - 8usize];
    [
        "Alignment of redundant_packed_bitfield",
    ][::std::mem::align_of::<redundant_packed_bitfield>() - 8usize];
    [
        "Offset of field: redundant_packed_bitfield::a",
    ][::std::mem::offset_of!(redundant_packed_bitfield, a) - 0usize];
    [
        "Offset of field: redundant_packed_bitfield::c",
    ][::std::mem::offset_of!(redundant_packed_bitfield, c) - 4usize];
};
impl redundant_packed_bitfield {
    #[inline]
    pub fn b0(&self) -> u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_b0(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn b0_raw(this: *const Self) -> u8 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 1usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 0usize, 1u8) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_b0_raw(this: *mut Self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 1usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn b1(&self) -> u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_b1(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn b1_raw(this: *const Self) -> u8 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 1usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 1usize, 1u8) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_b1_raw(this: *mut Self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 1usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(b0: u8, b1: u8) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                1u8,
                {
                    let b0: u8 = unsafe { ::std::mem::transmute(b0) };
                    b0 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                1usize,
                1u8,
                {
                    let b1: u8 = unsafe { ::std::mem::transmute(b1) };
                    b1 as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union redundant_packed_union {
    pub a: u64,
    pub b: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of redundant_packed_union",
    ][::std::mem::size_of::<redundant_packed_union>() - 16usize];
    [
        "Alignment of redundant_packed_union",
    ][::std::mem::align_of::<redundant_packed_union>() - 16usize];
    [
        "Offset of field: redundant_packed_union::a",
    ][::std::mem::offset_of!(redundant_packed_union, a) - 0usize];
    [
        "Offset of field: redundant_packed_union::b",
    ][::std::mem::offset_of!(redundant_packed_union, b) - 0usize];
};
impl Default for redundant_packed_union {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[repr(align(2))]
#[derive(Debug, Default, Copy, Clone)]
pub struct inner {
    pub a: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of inner"][::std::mem::size_of::<inner>() - 2usize];
    ["Alignment of inner"][::std::mem::align_of::<inner>() - 2usize];
    ["Offset of field: inner::a"][::std::mem::offset_of!(inner, a) - 0usize];
};
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct outer_redundant_packed {
    pub a: [inner; 2usize],
    pub b: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of outer_redundant_packed",
    ][::std::mem::size_of::<outer_redundant_packed>() - 8usize];
    [
        "Alignment of outer_redundant_packed",
    ][::std::mem::align_of::<outer_redundant_packed>() - 8usize];
    [
        "Offset of field: outer_redundant_packed::a",
    ][::std::mem::offset_of!(outer_redundant_packed, a) - 0usize];
    [
        "Offset of field: outer_redundant_packed::b",
    ][::std::mem::offset_of!(outer_redundant_packed, b) - 4usize];
};
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct redundant_pragma_packed {
    pub a: u8,
    pub b: u16,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of redundant_pragma_packed",
    ][::std::mem::size_of::<redundant_pragma_packed>() - 4usize];
    [
        "Alignment of redundant_pragma_packed",
    ][::std::mem::align_of::<redundant_pragma_packed>() - 4usize];
    [
        "Offset of field: redundant_pragma_packed::a",
    ][::std::mem::offset_of!(redundant_pragma_packed, a) - 0usize];
    [
        "Offset of field: redundant_pragma_packed::b",
    ][::std::mem::offset_of!(redundant_pragma_packed, b) - 2usize];
};
