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
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len(),
        );
        if bit_width == 0 {
            return 0;
        }
        let mut val = 0u64;
        let storage = self.storage.as_ref();
        let start_byte = bit_offset / 8;
        let bit_shift = bit_offset % 8;
        let bytes_needed = (bit_width as usize + bit_shift + 7) / 8;
        if cfg!(target_endian = "big") {
            for i in 0..bytes_needed {
                val |= (storage[start_byte + i].reverse_bits() as u64) << (i * 8);
            }
        } else {
            for i in 0..bytes_needed {
                val |= (storage[start_byte + i] as u64) << (i * 8);
            }
        }
        val >>= bit_shift;
        if bit_width < 64 {
            val &= (1u64 << bit_width) - 1;
        }
        if cfg!(target_endian = "big") {
            val = val.reverse_bits() >> (64 - bit_width as usize);
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
        if bit_width == 0 {
            return 0;
        }
        let mut val = 0u64;
        let start_byte = bit_offset / 8;
        let bit_shift = bit_offset % 8;
        let bytes_needed = (bit_width as usize + bit_shift + 7) / 8;
        let storage_ptr = unsafe { core::ptr::addr_of!((*this).storage) as *const u8 };
        if cfg!(target_endian = "big") {
            for i in 0..bytes_needed {
                let byte = unsafe { *storage_ptr.add(start_byte + i) };
                val |= (byte.reverse_bits() as u64) << (i * 8);
            }
        } else {
            for i in 0..bytes_needed {
                let byte = unsafe { *storage_ptr.add(start_byte + i) };
                val |= (byte as u64) << (i * 8);
            }
        }
        val >>= bit_shift;
        if bit_width < 64 {
            val &= (1u64 << bit_width) - 1;
        }
        if cfg!(target_endian = "big") {
            val = val.reverse_bits() >> (64 - bit_width as usize);
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
        if bit_width == 0 {
            return;
        }
        let mut val = val;
        if bit_width < 64 {
            val &= (1u64 << bit_width) - 1;
        }
        if cfg!(target_endian = "big") {
            val = val.reverse_bits() >> (64 - bit_width as usize);
        }
        let storage = self.storage.as_mut();
        let start_byte = bit_offset / 8;
        let bit_shift = bit_offset % 8;
        let bytes_needed = (bit_width as usize + bit_shift + 7) / 8;
        val <<= bit_shift;
        let field_mask = if bit_width as usize + bit_shift >= 64 {
            !0u64 << bit_shift
        } else {
            ((1u64 << bit_width) - 1) << bit_shift
        };
        for i in 0..bytes_needed {
            let byte_val = (val >> (i * 8)) as u8;
            let byte_mask = (field_mask >> (i * 8)) as u8;
            if cfg!(target_endian = "big") {
                let byte = storage[start_byte + i].reverse_bits();
                let new_byte = (byte & !byte_mask) | (byte_val & byte_mask);
                storage[start_byte + i] = new_byte.reverse_bits();
            } else {
                storage[start_byte + i] = (storage[start_byte + i] & !byte_mask)
                    | (byte_val & byte_mask);
            }
        }
    }
    #[inline]
    pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>(),
        );
        if bit_width == 0 {
            return;
        }
        let mut val = val;
        if bit_width < 64 {
            val &= (1u64 << bit_width) - 1;
        }
        if cfg!(target_endian = "big") {
            val = val.reverse_bits() >> (64 - bit_width as usize);
        }
        let start_byte = bit_offset / 8;
        let bit_shift = bit_offset % 8;
        let bytes_needed = (bit_width as usize + bit_shift + 7) / 8;
        val <<= bit_shift;
        let field_mask = if bit_width as usize + bit_shift >= 64 {
            !0u64 << bit_shift
        } else {
            ((1u64 << bit_width) - 1) << bit_shift
        };
        let storage_ptr = unsafe { core::ptr::addr_of_mut!((*this).storage) as *mut u8 };
        for i in 0..bytes_needed {
            let byte_val = (val >> (i * 8)) as u8;
            let byte_mask = (field_mask >> (i * 8)) as u8;
            let byte_ptr = unsafe { storage_ptr.add(start_byte + i) };
            if cfg!(target_endian = "big") {
                let byte = unsafe { (*byte_ptr).reverse_bits() };
                let new_byte = (byte & !byte_mask) | (byte_val & byte_mask);
                unsafe { *byte_ptr = new_byte.reverse_bits() };
            } else {
                unsafe { *byte_ptr = (*byte_ptr & !byte_mask) | (byte_val & byte_mask) };
            }
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Test {
    pub foo: u64,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
impl Test {
    #[inline]
    pub fn x(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 56u8) as u64) }
    }
    #[inline]
    pub fn set_x(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 56u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn x_raw(this: *const Self) -> u64 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 8usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 0usize, 56u8)
                    as u64,
            )
        }
    }
    #[inline]
    pub unsafe fn set_x_raw(this: *mut Self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 8usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                56u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn y(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(56usize, 8u8) as u64) }
    }
    #[inline]
    pub fn set_y(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(56usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn y_raw(this: *const Self) -> u64 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 8usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 56usize, 8u8)
                    as u64,
            )
        }
    }
    #[inline]
    pub unsafe fn set_y_raw(this: *mut Self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 8usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                56usize,
                8u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(x: u64, y: u64) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                56u8,
                {
                    let x: u64 = unsafe { ::std::mem::transmute(x) };
                    x as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                56usize,
                8u8,
                {
                    let y: u64 = unsafe { ::std::mem::transmute(y) };
                    y as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
