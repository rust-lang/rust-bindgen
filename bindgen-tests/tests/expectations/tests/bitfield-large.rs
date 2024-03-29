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
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone)]
pub struct HasBigBitfield {
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 16usize]>,
}
const _: () = {
    ["Size of HasBigBitfield"][::std::mem::size_of::<HasBigBitfield>() - 16usize];
    ["Alignment of HasBigBitfield"][::std::mem::align_of::<HasBigBitfield>() - 16usize];
};
impl HasBigBitfield {
    #[inline]
    pub fn x(&self) -> i128 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 128u8) as u128) }
    }
    #[inline]
    pub fn set_x(&mut self, val: i128) {
        unsafe {
            let val: u128 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 128u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(x: i128) -> __BindgenBitfieldUnit<[u8; 16usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 16usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                128u8,
                {
                    let x: u128 = unsafe { ::std::mem::transmute(x) };
                    x as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone)]
pub struct HasTwoBigBitfields {
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 16usize]>,
}
const _: () = {
    [
        "Size of HasTwoBigBitfields",
    ][::std::mem::size_of::<HasTwoBigBitfields>() - 16usize];
    [
        "Alignment of HasTwoBigBitfields",
    ][::std::mem::align_of::<HasTwoBigBitfields>() - 16usize];
};
impl HasTwoBigBitfields {
    #[inline]
    pub fn x(&self) -> i128 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 80u8) as u128) }
    }
    #[inline]
    pub fn set_x(&mut self, val: i128) {
        unsafe {
            let val: u128 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 80u8, val as u64)
        }
    }
    #[inline]
    pub fn y(&self) -> i128 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(80usize, 48u8) as u128) }
    }
    #[inline]
    pub fn set_y(&mut self, val: i128) {
        unsafe {
            let val: u128 = ::std::mem::transmute(val);
            self._bitfield_1.set(80usize, 48u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(x: i128, y: i128) -> __BindgenBitfieldUnit<[u8; 16usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 16usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                80u8,
                {
                    let x: u128 = unsafe { ::std::mem::transmute(x) };
                    x as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                80usize,
                48u8,
                {
                    let y: u128 = unsafe { ::std::mem::transmute(y) };
                    y as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
