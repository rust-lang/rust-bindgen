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
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct redundant_packed {
    pub a: u32,
    pub b: u32,
}
#[test]
fn bindgen_test_layout_redundant_packed() {
    const UNINIT: ::std::mem::MaybeUninit<redundant_packed> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<redundant_packed>(),
        8usize,
        concat!("Size of: ", stringify!(redundant_packed)),
    );
    assert_eq!(
        ::std::mem::align_of::<redundant_packed>(),
        8usize,
        concat!("Alignment of ", stringify!(redundant_packed)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(redundant_packed), "::", stringify!(a)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        4usize,
        concat!("Offset of field: ", stringify!(redundant_packed), "::", stringify!(b)),
    );
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct redundant_packed_bitfield {
    pub a: [u8; 3usize],
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub c: u32,
}
#[test]
fn bindgen_test_layout_redundant_packed_bitfield() {
    const UNINIT: ::std::mem::MaybeUninit<redundant_packed_bitfield> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<redundant_packed_bitfield>(),
        8usize,
        concat!("Size of: ", stringify!(redundant_packed_bitfield)),
    );
    assert_eq!(
        ::std::mem::align_of::<redundant_packed_bitfield>(),
        8usize,
        concat!("Alignment of ", stringify!(redundant_packed_bitfield)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(redundant_packed_bitfield),
            "::",
            stringify!(a),
        ),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).c) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(redundant_packed_bitfield),
            "::",
            stringify!(c),
        ),
    );
}
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
#[test]
fn bindgen_test_layout_redundant_packed_union() {
    const UNINIT: ::std::mem::MaybeUninit<redundant_packed_union> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<redundant_packed_union>(),
        16usize,
        concat!("Size of: ", stringify!(redundant_packed_union)),
    );
    assert_eq!(
        ::std::mem::align_of::<redundant_packed_union>(),
        16usize,
        concat!("Alignment of ", stringify!(redundant_packed_union)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(redundant_packed_union),
            "::",
            stringify!(a),
        ),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(redundant_packed_union),
            "::",
            stringify!(b),
        ),
    );
}
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
#[test]
fn bindgen_test_layout_inner() {
    const UNINIT: ::std::mem::MaybeUninit<inner> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<inner>(),
        2usize,
        concat!("Size of: ", stringify!(inner)),
    );
    assert_eq!(
        ::std::mem::align_of::<inner>(),
        2usize,
        concat!("Alignment of ", stringify!(inner)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(inner), "::", stringify!(a)),
    );
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct outer_redundant_packed {
    pub a: [inner; 2usize],
    pub b: u32,
}
#[test]
fn bindgen_test_layout_outer_redundant_packed() {
    const UNINIT: ::std::mem::MaybeUninit<outer_redundant_packed> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<outer_redundant_packed>(),
        8usize,
        concat!("Size of: ", stringify!(outer_redundant_packed)),
    );
    assert_eq!(
        ::std::mem::align_of::<outer_redundant_packed>(),
        8usize,
        concat!("Alignment of ", stringify!(outer_redundant_packed)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(outer_redundant_packed),
            "::",
            stringify!(a),
        ),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(outer_redundant_packed),
            "::",
            stringify!(b),
        ),
    );
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct redundant_pragma_packed {
    pub a: u8,
    pub b: u16,
}
#[test]
fn bindgen_test_layout_redundant_pragma_packed() {
    const UNINIT: ::std::mem::MaybeUninit<redundant_pragma_packed> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<redundant_pragma_packed>(),
        4usize,
        concat!("Size of: ", stringify!(redundant_pragma_packed)),
    );
    assert_eq!(
        ::std::mem::align_of::<redundant_pragma_packed>(),
        4usize,
        concat!("Alignment of ", stringify!(redundant_pragma_packed)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(redundant_pragma_packed),
            "::",
            stringify!(a),
        ),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(redundant_pragma_packed),
            "::",
            stringify!(b),
        ),
    );
}
