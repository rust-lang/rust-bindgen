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
/// Const-generic methods for efficient bitfield access when offset and width
/// are known at compile time.
impl<const N: usize> __BindgenBitfieldUnit<[u8; N]> {
    /// Get a field using const generics for compile-time optimization.
    /// Uses native word size operations when the field fits in usize.
    #[inline]
    pub const fn get_const<const BIT_OFFSET: usize, const BIT_WIDTH: u8>(&self) -> u64 {
        debug_assert!(BIT_WIDTH <= 64);
        debug_assert!(BIT_OFFSET / 8 < N);
        debug_assert!((BIT_OFFSET + (BIT_WIDTH as usize)) / 8 <= N);
        if BIT_WIDTH == 0 {
            return 0;
        }
        let start_byte = BIT_OFFSET / 8;
        let bit_shift = BIT_OFFSET % 8;
        let bytes_needed = (BIT_WIDTH as usize + bit_shift + 7) / 8;
        if BIT_WIDTH as usize + bit_shift <= usize::BITS as usize {
            let mut val = 0usize;
            if cfg!(target_endian = "big") {
                let mut i = 0;
                while i < bytes_needed {
                    val
                        |= (self.storage[start_byte + i].reverse_bits() as usize)
                            << (i * 8);
                    i += 1;
                }
            } else {
                let mut i = 0;
                while i < bytes_needed {
                    val |= (self.storage[start_byte + i] as usize) << (i * 8);
                    i += 1;
                }
            }
            val >>= bit_shift;
            val &= (1usize << BIT_WIDTH) - 1;
            if cfg!(target_endian = "big") {
                val = val.reverse_bits() >> (usize::BITS as usize - BIT_WIDTH as usize);
            }
            val as u64
        } else {
            let mut val = 0u64;
            if cfg!(target_endian = "big") {
                let mut i = 0;
                while i < bytes_needed {
                    val
                        |= (self.storage[start_byte + i].reverse_bits() as u64)
                            << (i * 8);
                    i += 1;
                }
            } else {
                let mut i = 0;
                while i < bytes_needed {
                    val |= (self.storage[start_byte + i] as u64) << (i * 8);
                    i += 1;
                }
            }
            val >>= bit_shift;
            if BIT_WIDTH < 64 {
                val &= (1u64 << BIT_WIDTH) - 1;
            }
            if cfg!(target_endian = "big") {
                val = val.reverse_bits() >> (64 - BIT_WIDTH as usize);
            }
            val
        }
    }
    /// Set a field using const generics for compile-time optimization.
    /// Uses native word size operations when the field fits in usize.
    #[inline]
    pub fn set_const<const BIT_OFFSET: usize, const BIT_WIDTH: u8>(&mut self, val: u64) {
        debug_assert!(BIT_WIDTH <= 64);
        debug_assert!(BIT_OFFSET / 8 < N);
        debug_assert!((BIT_OFFSET + (BIT_WIDTH as usize)) / 8 <= N);
        if BIT_WIDTH == 0 {
            return;
        }
        let start_byte = BIT_OFFSET / 8;
        let bit_shift = BIT_OFFSET % 8;
        let bytes_needed = (BIT_WIDTH as usize + bit_shift + 7) / 8;
        if BIT_WIDTH as usize + bit_shift <= usize::BITS as usize {
            let mut val = val as usize;
            val &= (1usize << BIT_WIDTH) - 1;
            if cfg!(target_endian = "big") {
                val = val.reverse_bits() >> (usize::BITS as usize - BIT_WIDTH as usize);
            }
            val <<= bit_shift;
            let field_mask = ((1usize << BIT_WIDTH) - 1) << bit_shift;
            let mut i = 0;
            while i < bytes_needed {
                let byte_val = (val >> (i * 8)) as u8;
                let byte_mask = (field_mask >> (i * 8)) as u8;
                if cfg!(target_endian = "big") {
                    let byte = self.storage[start_byte + i].reverse_bits();
                    let new_byte = (byte & !byte_mask) | (byte_val & byte_mask);
                    self.storage[start_byte + i] = new_byte.reverse_bits();
                } else {
                    self.storage[start_byte + i] = (self.storage[start_byte + i]
                        & !byte_mask) | (byte_val & byte_mask);
                }
                i += 1;
            }
        } else {
            let mut val = val;
            if BIT_WIDTH < 64 {
                val &= (1u64 << BIT_WIDTH) - 1;
            }
            if cfg!(target_endian = "big") {
                val = val.reverse_bits() >> (64 - BIT_WIDTH as usize);
            }
            val <<= bit_shift;
            let field_mask = if BIT_WIDTH as usize + bit_shift >= 64 {
                !0u64 << bit_shift
            } else {
                ((1u64 << BIT_WIDTH) - 1) << bit_shift
            };
            let mut i = 0;
            while i < bytes_needed {
                let byte_val = (val >> (i * 8)) as u8;
                let byte_mask = (field_mask >> (i * 8)) as u8;
                if cfg!(target_endian = "big") {
                    let byte = self.storage[start_byte + i].reverse_bits();
                    let new_byte = (byte & !byte_mask) | (byte_val & byte_mask);
                    self.storage[start_byte + i] = new_byte.reverse_bits();
                } else {
                    self.storage[start_byte + i] = (self.storage[start_byte + i]
                        & !byte_mask) | (byte_val & byte_mask);
                }
                i += 1;
            }
        }
    }
    /// Raw pointer get using const generics for compile-time optimization.
    /// Uses native word size operations when the field fits in usize.
    #[inline]
    pub const unsafe fn raw_get_const<const BIT_OFFSET: usize, const BIT_WIDTH: u8>(
        this: *const Self,
    ) -> u64 {
        debug_assert!(BIT_WIDTH <= 64);
        debug_assert!(BIT_OFFSET / 8 < N);
        debug_assert!((BIT_OFFSET + (BIT_WIDTH as usize)) / 8 <= N);
        if BIT_WIDTH == 0 {
            return 0;
        }
        let start_byte = BIT_OFFSET / 8;
        let bit_shift = BIT_OFFSET % 8;
        let bytes_needed = (BIT_WIDTH as usize + bit_shift + 7) / 8;
        let storage_ptr = unsafe { core::ptr::addr_of!((*this).storage) as *const u8 };
        if BIT_WIDTH as usize + bit_shift <= usize::BITS as usize {
            let mut val = 0usize;
            if cfg!(target_endian = "big") {
                let mut i = 0;
                while i < bytes_needed {
                    let byte = unsafe { *storage_ptr.add(start_byte + i) };
                    val |= (byte.reverse_bits() as usize) << (i * 8);
                    i += 1;
                }
            } else {
                let mut i = 0;
                while i < bytes_needed {
                    let byte = unsafe { *storage_ptr.add(start_byte + i) };
                    val |= (byte as usize) << (i * 8);
                    i += 1;
                }
            }
            val >>= bit_shift;
            val &= (1usize << BIT_WIDTH) - 1;
            if cfg!(target_endian = "big") {
                val = val.reverse_bits() >> (usize::BITS as usize - BIT_WIDTH as usize);
            }
            val as u64
        } else {
            let mut val = 0u64;
            if cfg!(target_endian = "big") {
                let mut i = 0;
                while i < bytes_needed {
                    let byte = unsafe { *storage_ptr.add(start_byte + i) };
                    val |= (byte.reverse_bits() as u64) << (i * 8);
                    i += 1;
                }
            } else {
                let mut i = 0;
                while i < bytes_needed {
                    let byte = unsafe { *storage_ptr.add(start_byte + i) };
                    val |= (byte as u64) << (i * 8);
                    i += 1;
                }
            }
            val >>= bit_shift;
            if BIT_WIDTH < 64 {
                val &= (1u64 << BIT_WIDTH) - 1;
            }
            if cfg!(target_endian = "big") {
                val = val.reverse_bits() >> (64 - BIT_WIDTH as usize);
            }
            val
        }
    }
    /// Raw pointer set using const generics for compile-time optimization.
    /// Uses native word size operations when the field fits in usize.
    #[inline]
    pub unsafe fn raw_set_const<const BIT_OFFSET: usize, const BIT_WIDTH: u8>(
        this: *mut Self,
        val: u64,
    ) {
        debug_assert!(BIT_WIDTH <= 64);
        debug_assert!(BIT_OFFSET / 8 < N);
        debug_assert!((BIT_OFFSET + (BIT_WIDTH as usize)) / 8 <= N);
        if BIT_WIDTH == 0 {
            return;
        }
        let start_byte = BIT_OFFSET / 8;
        let bit_shift = BIT_OFFSET % 8;
        let bytes_needed = (BIT_WIDTH as usize + bit_shift + 7) / 8;
        let storage_ptr = this.cast::<[u8; N]>().cast::<u8>();
        if BIT_WIDTH as usize + bit_shift <= usize::BITS as usize {
            let mut val = val as usize;
            val &= (1usize << BIT_WIDTH) - 1;
            if cfg!(target_endian = "big") {
                val = val.reverse_bits() >> (usize::BITS as usize - BIT_WIDTH as usize);
            }
            val <<= bit_shift;
            let field_mask = ((1usize << BIT_WIDTH) - 1) << bit_shift;
            let mut i = 0;
            while i < bytes_needed {
                let byte_val = (val >> (i * 8)) as u8;
                let byte_mask = (field_mask >> (i * 8)) as u8;
                let byte_ptr = unsafe { storage_ptr.add(start_byte + i) };
                if cfg!(target_endian = "big") {
                    let byte = unsafe { (*byte_ptr).reverse_bits() };
                    let new_byte = (byte & !byte_mask) | (byte_val & byte_mask);
                    unsafe { *byte_ptr = new_byte.reverse_bits() };
                } else {
                    unsafe {
                        *byte_ptr = (*byte_ptr & !byte_mask) | (byte_val & byte_mask)
                    };
                }
                i += 1;
            }
        } else {
            let mut val = val;
            if BIT_WIDTH < 64 {
                val &= (1u64 << BIT_WIDTH) - 1;
            }
            if cfg!(target_endian = "big") {
                val = val.reverse_bits() >> (64 - BIT_WIDTH as usize);
            }
            val <<= bit_shift;
            let field_mask = if BIT_WIDTH as usize + bit_shift >= 64 {
                !0u64 << bit_shift
            } else {
                ((1u64 << BIT_WIDTH) - 1) << bit_shift
            };
            let mut i = 0;
            while i < bytes_needed {
                let byte_val = (val >> (i * 8)) as u8;
                let byte_mask = (field_mask >> (i * 8)) as u8;
                let byte_ptr = unsafe { storage_ptr.add(start_byte + i) };
                if cfg!(target_endian = "big") {
                    let byte = unsafe { (*byte_ptr).reverse_bits() };
                    let new_byte = (byte & !byte_mask) | (byte_val & byte_mask);
                    unsafe { *byte_ptr = new_byte.reverse_bits() };
                } else {
                    unsafe {
                        *byte_ptr = (*byte_ptr & !byte_mask) | (byte_val & byte_mask)
                    };
                }
                i += 1;
            }
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct MuchBitfield {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 5usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MuchBitfield"][::std::mem::size_of::<MuchBitfield>() - 5usize];
    ["Alignment of MuchBitfield"][::std::mem::align_of::<MuchBitfield>() - 1usize];
};
impl MuchBitfield {
    #[inline]
    pub fn m0(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<0usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m0(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<0usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m0_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    0usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m0_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                0usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m1(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<1usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m1(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<1usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m1_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    1usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m1_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                1usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m2(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<2usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m2(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<2usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m2_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    2usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m2_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                2usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m3(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<3usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m3(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<3usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m3_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    3usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m3_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                3usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m4(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<4usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m4(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<4usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m4_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    4usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m4_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                4usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m5(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<5usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m5(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<5usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m5_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    5usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m5_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                5usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m6(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<6usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m6(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<6usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m6_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    6usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m6_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                6usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m7(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<7usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m7(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<7usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m7_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    7usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m7_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                7usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m8(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<8usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m8(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<8usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m8_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    8usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m8_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                8usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m9(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<9usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m9(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<9usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m9_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    9usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m9_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                9usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m10(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<10usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m10(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<10usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m10_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    10usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m10_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                10usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m11(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<11usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m11(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<11usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m11_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    11usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m11_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                11usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m12(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<12usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m12(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<12usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m12_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    12usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m12_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                12usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m13(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<13usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m13(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<13usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m13_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    13usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m13_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                13usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m14(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<14usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m14(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<14usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m14_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    14usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m14_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                14usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m15(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<15usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m15(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<15usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m15_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    15usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m15_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                15usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m16(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<16usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m16(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<16usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m16_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    16usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m16_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                16usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m17(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<17usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m17(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<17usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m17_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    17usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m17_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                17usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m18(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<18usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m18(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<18usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m18_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    18usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m18_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                18usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m19(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<19usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m19(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<19usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m19_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    19usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m19_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                19usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m20(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<20usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m20(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<20usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m20_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    20usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m20_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                20usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m21(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<21usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m21(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<21usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m21_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    21usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m21_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                21usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m22(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<22usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m22(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<22usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m22_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    22usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m22_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                22usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m23(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<23usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m23(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<23usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m23_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    23usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m23_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                23usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m24(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<24usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m24(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<24usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m24_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    24usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m24_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                24usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m25(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<25usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m25(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<25usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m25_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    25usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m25_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                25usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m26(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<26usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m26(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<26usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m26_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    26usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m26_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                26usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m27(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<27usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m27(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<27usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m27_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    27usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m27_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                27usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m28(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<28usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m28(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<28usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m28_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    28usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m28_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                28usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m29(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<29usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m29(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<29usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m29_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    29usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m29_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                29usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m30(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<30usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m30(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<30usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m30_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    30usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m30_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                30usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m31(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<31usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m31(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<31usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m31_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    31usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m31_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                31usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn m32(&self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<32usize, 1u8>() as u8)
        }
    }
    #[inline]
    pub fn set_m32(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<32usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn m32_raw(this: *const Self) -> ::std::os::raw::c_char {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 5usize],
                >>::raw_get_const::<
                    32usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m32_raw(this: *mut Self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 5usize],
            >>::raw_set_const::<
                32usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
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
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 5usize]> = Default::default();
        __bindgen_bitfield_unit
            .set_const::<
                0usize,
                1u8,
            >({
                let m0: u8 = unsafe { ::std::mem::transmute(m0) };
                m0 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                1usize,
                1u8,
            >({
                let m1: u8 = unsafe { ::std::mem::transmute(m1) };
                m1 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                2usize,
                1u8,
            >({
                let m2: u8 = unsafe { ::std::mem::transmute(m2) };
                m2 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                3usize,
                1u8,
            >({
                let m3: u8 = unsafe { ::std::mem::transmute(m3) };
                m3 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                4usize,
                1u8,
            >({
                let m4: u8 = unsafe { ::std::mem::transmute(m4) };
                m4 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                5usize,
                1u8,
            >({
                let m5: u8 = unsafe { ::std::mem::transmute(m5) };
                m5 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                6usize,
                1u8,
            >({
                let m6: u8 = unsafe { ::std::mem::transmute(m6) };
                m6 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                7usize,
                1u8,
            >({
                let m7: u8 = unsafe { ::std::mem::transmute(m7) };
                m7 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                8usize,
                1u8,
            >({
                let m8: u8 = unsafe { ::std::mem::transmute(m8) };
                m8 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                9usize,
                1u8,
            >({
                let m9: u8 = unsafe { ::std::mem::transmute(m9) };
                m9 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                10usize,
                1u8,
            >({
                let m10: u8 = unsafe { ::std::mem::transmute(m10) };
                m10 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                11usize,
                1u8,
            >({
                let m11: u8 = unsafe { ::std::mem::transmute(m11) };
                m11 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                12usize,
                1u8,
            >({
                let m12: u8 = unsafe { ::std::mem::transmute(m12) };
                m12 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                13usize,
                1u8,
            >({
                let m13: u8 = unsafe { ::std::mem::transmute(m13) };
                m13 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                14usize,
                1u8,
            >({
                let m14: u8 = unsafe { ::std::mem::transmute(m14) };
                m14 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                15usize,
                1u8,
            >({
                let m15: u8 = unsafe { ::std::mem::transmute(m15) };
                m15 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                16usize,
                1u8,
            >({
                let m16: u8 = unsafe { ::std::mem::transmute(m16) };
                m16 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                17usize,
                1u8,
            >({
                let m17: u8 = unsafe { ::std::mem::transmute(m17) };
                m17 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                18usize,
                1u8,
            >({
                let m18: u8 = unsafe { ::std::mem::transmute(m18) };
                m18 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                19usize,
                1u8,
            >({
                let m19: u8 = unsafe { ::std::mem::transmute(m19) };
                m19 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                20usize,
                1u8,
            >({
                let m20: u8 = unsafe { ::std::mem::transmute(m20) };
                m20 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                21usize,
                1u8,
            >({
                let m21: u8 = unsafe { ::std::mem::transmute(m21) };
                m21 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                22usize,
                1u8,
            >({
                let m22: u8 = unsafe { ::std::mem::transmute(m22) };
                m22 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                23usize,
                1u8,
            >({
                let m23: u8 = unsafe { ::std::mem::transmute(m23) };
                m23 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                24usize,
                1u8,
            >({
                let m24: u8 = unsafe { ::std::mem::transmute(m24) };
                m24 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                25usize,
                1u8,
            >({
                let m25: u8 = unsafe { ::std::mem::transmute(m25) };
                m25 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                26usize,
                1u8,
            >({
                let m26: u8 = unsafe { ::std::mem::transmute(m26) };
                m26 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                27usize,
                1u8,
            >({
                let m27: u8 = unsafe { ::std::mem::transmute(m27) };
                m27 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                28usize,
                1u8,
            >({
                let m28: u8 = unsafe { ::std::mem::transmute(m28) };
                m28 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                29usize,
                1u8,
            >({
                let m29: u8 = unsafe { ::std::mem::transmute(m29) };
                m29 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                30usize,
                1u8,
            >({
                let m30: u8 = unsafe { ::std::mem::transmute(m30) };
                m30 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                31usize,
                1u8,
            >({
                let m31: u8 = unsafe { ::std::mem::transmute(m31) };
                m31 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                32usize,
                1u8,
            >({
                let m32: u8 = unsafe { ::std::mem::transmute(m32) };
                m32 as u64
            });
        __bindgen_bitfield_unit
    }
}
