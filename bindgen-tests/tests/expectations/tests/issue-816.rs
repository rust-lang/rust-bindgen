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
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct capabilities {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 16usize]>,
}
const _: () = {
    ["Size of capabilities"][::std::mem::size_of::<capabilities>() - 16usize];
    ["Alignment of capabilities"][::std::mem::align_of::<capabilities>() - 4usize];
};
impl capabilities {
    #[inline]
    pub fn bit_1(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_1(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_2(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_2(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_3(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_3(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_4(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_4(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_5(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_5(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_6(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_6(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_7(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_7(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_8(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_8(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_9(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_9(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_10(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_10(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(9usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_11(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(10usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_11(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(10usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_12(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(11usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_12(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(11usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_13(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(12usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_13(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(12usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_14(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_14(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_15(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(14usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_15(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(14usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_16(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(15usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_16(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(15usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_17(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_17(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_18(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(17usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_18(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(17usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_19(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(18usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_19(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(18usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_20(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(19usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_20(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(19usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_21(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(20usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_21(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(20usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_22(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(21usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_22(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(21usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_23(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(22usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_23(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(22usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_24(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(23usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_24(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(23usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_25(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(24usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_25(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(24usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_26(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(25usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_26(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(25usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_27(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(26usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_27(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(26usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_28(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(27usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_28(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(27usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_29(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(28usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_29(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(28usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_30(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(29usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_30(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(29usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_31(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(30usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_31(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(30usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_32(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(31usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_32(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(31usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_33(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(32usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_33(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(32usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_34(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(33usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_34(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(33usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_35(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(34usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_35(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(34usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_36(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(35usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_36(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(35usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_37(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(36usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_37(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(36usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_38(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(37usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_38(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(37usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_39(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(38usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_39(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(38usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_40(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(39usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_40(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(39usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn bit_41(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(40usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bit_41(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(40usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        bit_1: ::std::os::raw::c_uint,
        bit_2: ::std::os::raw::c_uint,
        bit_3: ::std::os::raw::c_uint,
        bit_4: ::std::os::raw::c_uint,
        bit_5: ::std::os::raw::c_uint,
        bit_6: ::std::os::raw::c_uint,
        bit_7: ::std::os::raw::c_uint,
        bit_8: ::std::os::raw::c_uint,
        bit_9: ::std::os::raw::c_uint,
        bit_10: ::std::os::raw::c_uint,
        bit_11: ::std::os::raw::c_uint,
        bit_12: ::std::os::raw::c_uint,
        bit_13: ::std::os::raw::c_uint,
        bit_14: ::std::os::raw::c_uint,
        bit_15: ::std::os::raw::c_uint,
        bit_16: ::std::os::raw::c_uint,
        bit_17: ::std::os::raw::c_uint,
        bit_18: ::std::os::raw::c_uint,
        bit_19: ::std::os::raw::c_uint,
        bit_20: ::std::os::raw::c_uint,
        bit_21: ::std::os::raw::c_uint,
        bit_22: ::std::os::raw::c_uint,
        bit_23: ::std::os::raw::c_uint,
        bit_24: ::std::os::raw::c_uint,
        bit_25: ::std::os::raw::c_uint,
        bit_26: ::std::os::raw::c_uint,
        bit_27: ::std::os::raw::c_uint,
        bit_28: ::std::os::raw::c_uint,
        bit_29: ::std::os::raw::c_uint,
        bit_30: ::std::os::raw::c_uint,
        bit_31: ::std::os::raw::c_uint,
        bit_32: ::std::os::raw::c_uint,
        bit_33: ::std::os::raw::c_uint,
        bit_34: ::std::os::raw::c_uint,
        bit_35: ::std::os::raw::c_uint,
        bit_36: ::std::os::raw::c_uint,
        bit_37: ::std::os::raw::c_uint,
        bit_38: ::std::os::raw::c_uint,
        bit_39: ::std::os::raw::c_uint,
        bit_40: ::std::os::raw::c_uint,
        bit_41: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 16usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 16usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                1u8,
                {
                    let bit_1: u32 = unsafe { ::std::mem::transmute(bit_1) };
                    bit_1 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                1usize,
                1u8,
                {
                    let bit_2: u32 = unsafe { ::std::mem::transmute(bit_2) };
                    bit_2 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                2usize,
                1u8,
                {
                    let bit_3: u32 = unsafe { ::std::mem::transmute(bit_3) };
                    bit_3 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                3usize,
                1u8,
                {
                    let bit_4: u32 = unsafe { ::std::mem::transmute(bit_4) };
                    bit_4 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                4usize,
                1u8,
                {
                    let bit_5: u32 = unsafe { ::std::mem::transmute(bit_5) };
                    bit_5 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                5usize,
                1u8,
                {
                    let bit_6: u32 = unsafe { ::std::mem::transmute(bit_6) };
                    bit_6 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                6usize,
                1u8,
                {
                    let bit_7: u32 = unsafe { ::std::mem::transmute(bit_7) };
                    bit_7 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                7usize,
                1u8,
                {
                    let bit_8: u32 = unsafe { ::std::mem::transmute(bit_8) };
                    bit_8 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                8usize,
                1u8,
                {
                    let bit_9: u32 = unsafe { ::std::mem::transmute(bit_9) };
                    bit_9 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                9usize,
                1u8,
                {
                    let bit_10: u32 = unsafe { ::std::mem::transmute(bit_10) };
                    bit_10 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                10usize,
                1u8,
                {
                    let bit_11: u32 = unsafe { ::std::mem::transmute(bit_11) };
                    bit_11 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                11usize,
                1u8,
                {
                    let bit_12: u32 = unsafe { ::std::mem::transmute(bit_12) };
                    bit_12 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                12usize,
                1u8,
                {
                    let bit_13: u32 = unsafe { ::std::mem::transmute(bit_13) };
                    bit_13 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                13usize,
                1u8,
                {
                    let bit_14: u32 = unsafe { ::std::mem::transmute(bit_14) };
                    bit_14 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                14usize,
                1u8,
                {
                    let bit_15: u32 = unsafe { ::std::mem::transmute(bit_15) };
                    bit_15 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                15usize,
                1u8,
                {
                    let bit_16: u32 = unsafe { ::std::mem::transmute(bit_16) };
                    bit_16 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                16usize,
                1u8,
                {
                    let bit_17: u32 = unsafe { ::std::mem::transmute(bit_17) };
                    bit_17 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                17usize,
                1u8,
                {
                    let bit_18: u32 = unsafe { ::std::mem::transmute(bit_18) };
                    bit_18 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                18usize,
                1u8,
                {
                    let bit_19: u32 = unsafe { ::std::mem::transmute(bit_19) };
                    bit_19 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                19usize,
                1u8,
                {
                    let bit_20: u32 = unsafe { ::std::mem::transmute(bit_20) };
                    bit_20 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                20usize,
                1u8,
                {
                    let bit_21: u32 = unsafe { ::std::mem::transmute(bit_21) };
                    bit_21 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                21usize,
                1u8,
                {
                    let bit_22: u32 = unsafe { ::std::mem::transmute(bit_22) };
                    bit_22 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                22usize,
                1u8,
                {
                    let bit_23: u32 = unsafe { ::std::mem::transmute(bit_23) };
                    bit_23 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                23usize,
                1u8,
                {
                    let bit_24: u32 = unsafe { ::std::mem::transmute(bit_24) };
                    bit_24 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                24usize,
                1u8,
                {
                    let bit_25: u32 = unsafe { ::std::mem::transmute(bit_25) };
                    bit_25 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                25usize,
                1u8,
                {
                    let bit_26: u32 = unsafe { ::std::mem::transmute(bit_26) };
                    bit_26 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                26usize,
                1u8,
                {
                    let bit_27: u32 = unsafe { ::std::mem::transmute(bit_27) };
                    bit_27 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                27usize,
                1u8,
                {
                    let bit_28: u32 = unsafe { ::std::mem::transmute(bit_28) };
                    bit_28 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                28usize,
                1u8,
                {
                    let bit_29: u32 = unsafe { ::std::mem::transmute(bit_29) };
                    bit_29 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                29usize,
                1u8,
                {
                    let bit_30: u32 = unsafe { ::std::mem::transmute(bit_30) };
                    bit_30 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                30usize,
                1u8,
                {
                    let bit_31: u32 = unsafe { ::std::mem::transmute(bit_31) };
                    bit_31 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                31usize,
                1u8,
                {
                    let bit_32: u32 = unsafe { ::std::mem::transmute(bit_32) };
                    bit_32 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                32usize,
                1u8,
                {
                    let bit_33: u32 = unsafe { ::std::mem::transmute(bit_33) };
                    bit_33 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                33usize,
                1u8,
                {
                    let bit_34: u32 = unsafe { ::std::mem::transmute(bit_34) };
                    bit_34 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                34usize,
                1u8,
                {
                    let bit_35: u32 = unsafe { ::std::mem::transmute(bit_35) };
                    bit_35 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                35usize,
                1u8,
                {
                    let bit_36: u32 = unsafe { ::std::mem::transmute(bit_36) };
                    bit_36 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                36usize,
                1u8,
                {
                    let bit_37: u32 = unsafe { ::std::mem::transmute(bit_37) };
                    bit_37 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                37usize,
                1u8,
                {
                    let bit_38: u32 = unsafe { ::std::mem::transmute(bit_38) };
                    bit_38 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                38usize,
                1u8,
                {
                    let bit_39: u32 = unsafe { ::std::mem::transmute(bit_39) };
                    bit_39 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                39usize,
                1u8,
                {
                    let bit_40: u32 = unsafe { ::std::mem::transmute(bit_40) };
                    bit_40 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                40usize,
                1u8,
                {
                    let bit_41: u32 = unsafe { ::std::mem::transmute(bit_41) };
                    bit_41 as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
