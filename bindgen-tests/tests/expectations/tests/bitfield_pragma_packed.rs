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
#[derive(Debug, Default, Copy, Clone)]
pub struct Struct {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
const _: () = {
    ["Size of Struct"][::std::mem::size_of::<Struct>() - 4usize];
    ["Alignment of Struct"][::std::mem::align_of::<Struct>() - 1usize];
};
impl Struct {
    #[inline]
    pub fn a(&self) -> ::std::os::raw::c_uchar {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_a(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn b(&self) -> ::std::os::raw::c_uchar {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_b(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn c(&self) -> ::std::os::raw::c_uchar {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 6u8) as u8) }
    }
    #[inline]
    pub fn set_c(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub fn d(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 16u8) as u16) }
    }
    #[inline]
    pub fn set_d(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn e(&self) -> ::std::os::raw::c_uchar {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(24usize, 8u8) as u8) }
    }
    #[inline]
    pub fn set_e(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(24usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        a: ::std::os::raw::c_uchar,
        b: ::std::os::raw::c_uchar,
        c: ::std::os::raw::c_uchar,
        d: ::std::os::raw::c_ushort,
        e: ::std::os::raw::c_uchar,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                1u8,
                {
                    let a: u8 = unsafe { ::std::mem::transmute(a) };
                    a as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                1usize,
                1u8,
                {
                    let b: u8 = unsafe { ::std::mem::transmute(b) };
                    b as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                2usize,
                6u8,
                {
                    let c: u8 = unsafe { ::std::mem::transmute(c) };
                    c as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                8usize,
                16u8,
                {
                    let d: u16 = unsafe { ::std::mem::transmute(d) };
                    d as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                24usize,
                8u8,
                {
                    let e: u8 = unsafe { ::std::mem::transmute(e) };
                    e as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Inner {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
const _: () = {
    ["Size of Inner"][::std::mem::size_of::<Inner>() - 4usize];
    ["Alignment of Inner"][::std::mem::align_of::<Inner>() - 2usize];
};
impl Inner {
    #[inline]
    pub fn a(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 16u8) as u16) }
    }
    #[inline]
    pub fn set_a(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn b(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u16) }
    }
    #[inline]
    pub fn set_b(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        a: ::std::os::raw::c_ushort,
        b: ::std::os::raw::c_ushort,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                16u8,
                {
                    let a: u16 = unsafe { ::std::mem::transmute(a) };
                    a as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                16usize,
                16u8,
                {
                    let b: u16 = unsafe { ::std::mem::transmute(b) };
                    b as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Outer {
    pub inner: Inner,
}
const _: () = {
    ["Size of Outer"][::std::mem::size_of::<Outer>() - 4usize];
    ["Alignment of Outer"][::std::mem::align_of::<Outer>() - 1usize];
    ["Offset of field: Outer::inner"][::std::mem::offset_of!(Outer, inner) - 0usize];
};
