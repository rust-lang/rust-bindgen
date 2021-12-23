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
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct MuchBitfield {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 5usize]>,
}
#[test]
fn bindgen_test_layout_MuchBitfield() {
    assert_eq!(
        ::std::mem::size_of::<MuchBitfield>(),
        5usize,
        concat!("Size of: ", stringify!(MuchBitfield))
    );
    assert_eq!(
        ::std::mem::align_of::<MuchBitfield>(),
        1usize,
        concat!("Alignment of ", stringify!(MuchBitfield))
    );
}
impl MuchBitfield {
    #[inline]
    pub fn m0(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m0(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m1(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m1(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m2(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m2(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m3(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m3(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m4(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m4(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m5(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m5(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m6(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m6(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m7(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m7(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m8(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m8(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m9(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m9(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(9usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m10(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(10usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m10(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(10usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m11(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(11usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m11(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(11usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m12(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(12usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m12(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(12usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m13(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(13usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m13(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m14(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(14usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m14(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(14usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m15(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(15usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m15(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(15usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m16(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(16usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m16(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m17(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(17usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m17(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(17usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m18(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(18usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m18(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(18usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m19(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(19usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m19(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(19usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m20(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(20usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m20(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(20usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m21(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(21usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m21(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(21usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m22(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(22usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m22(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(22usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m23(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(23usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m23(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(23usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m24(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(24usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m24(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(24usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m25(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(25usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m25(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(25usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m26(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(26usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m26(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(26usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m27(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(27usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m27(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(27usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m28(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(28usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m28(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(28usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m29(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(29usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m29(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(29usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m30(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(30usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m30(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(30usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m31(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(31usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m31(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(31usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn m32(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(32usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_m32(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(32usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        m0: ::std::os::raw::c_char,
        m1: ::std::os::raw::c_char,
        m2: ::std::os::raw::c_char,
        m3: ::std::os::raw::c_char,
        m4: ::std::os::raw::c_char,
        m5: ::std::os::raw::c_char,
        m6: ::std::os::raw::c_char,
        m7: ::std::os::raw::c_char,
        m8: ::std::os::raw::c_char,
        m9: ::std::os::raw::c_char,
        m10: ::std::os::raw::c_char,
        m11: ::std::os::raw::c_char,
        m12: ::std::os::raw::c_char,
        m13: ::std::os::raw::c_char,
        m14: ::std::os::raw::c_char,
        m15: ::std::os::raw::c_char,
        m16: ::std::os::raw::c_char,
        m17: ::std::os::raw::c_char,
        m18: ::std::os::raw::c_char,
        m19: ::std::os::raw::c_char,
        m20: ::std::os::raw::c_char,
        m21: ::std::os::raw::c_char,
        m22: ::std::os::raw::c_char,
        m23: ::std::os::raw::c_char,
        m24: ::std::os::raw::c_char,
        m25: ::std::os::raw::c_char,
        m26: ::std::os::raw::c_char,
        m27: ::std::os::raw::c_char,
        m28: ::std::os::raw::c_char,
        m29: ::std::os::raw::c_char,
        m30: ::std::os::raw::c_char,
        m31: ::std::os::raw::c_char,
        m32: ::std::os::raw::c_char,
    ) -> __BindgenBitfieldUnit<[u8; 5usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 5usize]> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let m0: u8 = unsafe { ::std::mem::transmute(m0) };
            m0 as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let m1: u8 = unsafe { ::std::mem::transmute(m1) };
            m1 as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let m2: u8 = unsafe { ::std::mem::transmute(m2) };
            m2 as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let m3: u8 = unsafe { ::std::mem::transmute(m3) };
            m3 as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let m4: u8 = unsafe { ::std::mem::transmute(m4) };
            m4 as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let m5: u8 = unsafe { ::std::mem::transmute(m5) };
            m5 as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let m6: u8 = unsafe { ::std::mem::transmute(m6) };
            m6 as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let m7: u8 = unsafe { ::std::mem::transmute(m7) };
            m7 as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let m8: u8 = unsafe { ::std::mem::transmute(m8) };
            m8 as u64
        });
        __bindgen_bitfield_unit.set(9usize, 1u8, {
            let m9: u8 = unsafe { ::std::mem::transmute(m9) };
            m9 as u64
        });
        __bindgen_bitfield_unit.set(10usize, 1u8, {
            let m10: u8 = unsafe { ::std::mem::transmute(m10) };
            m10 as u64
        });
        __bindgen_bitfield_unit.set(11usize, 1u8, {
            let m11: u8 = unsafe { ::std::mem::transmute(m11) };
            m11 as u64
        });
        __bindgen_bitfield_unit.set(12usize, 1u8, {
            let m12: u8 = unsafe { ::std::mem::transmute(m12) };
            m12 as u64
        });
        __bindgen_bitfield_unit.set(13usize, 1u8, {
            let m13: u8 = unsafe { ::std::mem::transmute(m13) };
            m13 as u64
        });
        __bindgen_bitfield_unit.set(14usize, 1u8, {
            let m14: u8 = unsafe { ::std::mem::transmute(m14) };
            m14 as u64
        });
        __bindgen_bitfield_unit.set(15usize, 1u8, {
            let m15: u8 = unsafe { ::std::mem::transmute(m15) };
            m15 as u64
        });
        __bindgen_bitfield_unit.set(16usize, 1u8, {
            let m16: u8 = unsafe { ::std::mem::transmute(m16) };
            m16 as u64
        });
        __bindgen_bitfield_unit.set(17usize, 1u8, {
            let m17: u8 = unsafe { ::std::mem::transmute(m17) };
            m17 as u64
        });
        __bindgen_bitfield_unit.set(18usize, 1u8, {
            let m18: u8 = unsafe { ::std::mem::transmute(m18) };
            m18 as u64
        });
        __bindgen_bitfield_unit.set(19usize, 1u8, {
            let m19: u8 = unsafe { ::std::mem::transmute(m19) };
            m19 as u64
        });
        __bindgen_bitfield_unit.set(20usize, 1u8, {
            let m20: u8 = unsafe { ::std::mem::transmute(m20) };
            m20 as u64
        });
        __bindgen_bitfield_unit.set(21usize, 1u8, {
            let m21: u8 = unsafe { ::std::mem::transmute(m21) };
            m21 as u64
        });
        __bindgen_bitfield_unit.set(22usize, 1u8, {
            let m22: u8 = unsafe { ::std::mem::transmute(m22) };
            m22 as u64
        });
        __bindgen_bitfield_unit.set(23usize, 1u8, {
            let m23: u8 = unsafe { ::std::mem::transmute(m23) };
            m23 as u64
        });
        __bindgen_bitfield_unit.set(24usize, 1u8, {
            let m24: u8 = unsafe { ::std::mem::transmute(m24) };
            m24 as u64
        });
        __bindgen_bitfield_unit.set(25usize, 1u8, {
            let m25: u8 = unsafe { ::std::mem::transmute(m25) };
            m25 as u64
        });
        __bindgen_bitfield_unit.set(26usize, 1u8, {
            let m26: u8 = unsafe { ::std::mem::transmute(m26) };
            m26 as u64
        });
        __bindgen_bitfield_unit.set(27usize, 1u8, {
            let m27: u8 = unsafe { ::std::mem::transmute(m27) };
            m27 as u64
        });
        __bindgen_bitfield_unit.set(28usize, 1u8, {
            let m28: u8 = unsafe { ::std::mem::transmute(m28) };
            m28 as u64
        });
        __bindgen_bitfield_unit.set(29usize, 1u8, {
            let m29: u8 = unsafe { ::std::mem::transmute(m29) };
            m29 as u64
        });
        __bindgen_bitfield_unit.set(30usize, 1u8, {
            let m30: u8 = unsafe { ::std::mem::transmute(m30) };
            m30 as u64
        });
        __bindgen_bitfield_unit.set(31usize, 1u8, {
            let m31: u8 = unsafe { ::std::mem::transmute(m31) };
            m31 as u64
        });
        __bindgen_bitfield_unit.set(32usize, 1u8, {
            let m32: u8 = unsafe { ::std::mem::transmute(m32) };
            m32 as u64
        });
        __bindgen_bitfield_unit
    }
}
