#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

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
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <=
                self.storage.as_ref().len()
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
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <=
                self.storage.as_ref().len()
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
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union foo {
    pub a: ::std::os::raw::c_int,
    pub __bindgen_anon_1: foo__bindgen_ty_1,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo__bindgen_ty_1 {
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<foo__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(foo__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<foo__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(foo__bindgen_ty_1))
    );
}
impl foo__bindgen_ty_1 {
    #[inline]
    pub fn b(&self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 7u8) as u32)
        }
    }
    #[inline]
    pub fn set_b(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub fn c(&self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(7usize, 25u8) as u32)
        }
    }
    #[inline]
    pub fn set_c(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 25u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        b: ::std::os::raw::c_int,
        c: ::std::os::raw::c_int,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 7u8, {
            let b: u32 = unsafe { ::std::mem::transmute(b) };
            b as u64
        });
        __bindgen_bitfield_unit.set(7usize, 25u8, {
            let c: u32 = unsafe { ::std::mem::transmute(c) };
            c as u64
        });
        __bindgen_bitfield_unit
    }
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        4usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        4usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<foo>())).a as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(a))
    );
}
impl Default for foo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
