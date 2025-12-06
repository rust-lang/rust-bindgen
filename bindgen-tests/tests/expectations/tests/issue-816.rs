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
pub struct capabilities {
    pub _bindgen_align: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 16usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of capabilities"][::std::mem::size_of::<capabilities>() - 16usize];
    ["Alignment of capabilities"][::std::mem::align_of::<capabilities>() - 4usize];
};
impl capabilities {
    #[inline]
    pub fn bit_1(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<0usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_1(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<0usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_1_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    0usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_1_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                0usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_2(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<1usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_2(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<1usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_2_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    1usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_2_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                1usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_3(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<2usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_3(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<2usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_3_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    2usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_3_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                2usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_4(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<3usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_4(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<3usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_4_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    3usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_4_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                3usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_5(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<4usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_5(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<4usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_5_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    4usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_5_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                4usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_6(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<5usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_6(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<5usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_6_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    5usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_6_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                5usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_7(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<6usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_7(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<6usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_7_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    6usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_7_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                6usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_8(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<7usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_8(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<7usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_8_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    7usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_8_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                7usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_9(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<8usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_9(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<8usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_9_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    8usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_9_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                8usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_10(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<9usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_10(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<9usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_10_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    9usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_10_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                9usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_11(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<10usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_11(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<10usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_11_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    10usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_11_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                10usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_12(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<11usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_12(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<11usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_12_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    11usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_12_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                11usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_13(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<12usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_13(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<12usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_13_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    12usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_13_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                12usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_14(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<13usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_14(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<13usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_14_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    13usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_14_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                13usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_15(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<14usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_15(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<14usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_15_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    14usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_15_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                14usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_16(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<15usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_16(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<15usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_16_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    15usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_16_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                15usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_17(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<16usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_17(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<16usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_17_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    16usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_17_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                16usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_18(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<17usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_18(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<17usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_18_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    17usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_18_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                17usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_19(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<18usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_19(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<18usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_19_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    18usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_19_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                18usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_20(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<19usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_20(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<19usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_20_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    19usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_20_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                19usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_21(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<20usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_21(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<20usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_21_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    20usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_21_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                20usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_22(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<21usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_22(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<21usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_22_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    21usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_22_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                21usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_23(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<22usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_23(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<22usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_23_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    22usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_23_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                22usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_24(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<23usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_24(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<23usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_24_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    23usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_24_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                23usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_25(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<24usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_25(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<24usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_25_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    24usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_25_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                24usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_26(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<25usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_26(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<25usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_26_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    25usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_26_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                25usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_27(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<26usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_27(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<26usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_27_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    26usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_27_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                26usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_28(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<27usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_28(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<27usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_28_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    27usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_28_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                27usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_29(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<28usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_29(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<28usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_29_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    28usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_29_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                28usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_30(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<29usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_30(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<29usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_30_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    29usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_30_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                29usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_31(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<30usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_31(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<30usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_31_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    30usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_31_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                30usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_32(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<31usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_32(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<31usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_32_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    31usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_32_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                31usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_33(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<32usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_33(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<32usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_33_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    32usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_33_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                32usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_34(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<33usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_34(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<33usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_34_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    33usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_34_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                33usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_35(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<34usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_35(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<34usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_35_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    34usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_35_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                34usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_36(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<35usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_36(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<35usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_36_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    35usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_36_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                35usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_37(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<36usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_37(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<36usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_37_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    36usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_37_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                36usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_38(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<37usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_38(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<37usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_38_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    37usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_38_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                37usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_39(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<38usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_39(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<38usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_39_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    38usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_39_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                38usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_40(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<39usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_40(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<39usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_40_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    39usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_40_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                39usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn bit_41(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<40usize, 1u8>() as u32)
        }
    }
    #[inline]
    pub fn set_bit_41(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<40usize, 1u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn bit_41_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 16usize],
                >>::raw_get_const::<
                    40usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bit_41_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 16usize],
            >>::raw_set_const::<
                40usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
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
            .set_const::<
                0usize,
                1u8,
            >({
                let bit_1: u32 = unsafe { ::std::mem::transmute(bit_1) };
                bit_1 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                1usize,
                1u8,
            >({
                let bit_2: u32 = unsafe { ::std::mem::transmute(bit_2) };
                bit_2 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                2usize,
                1u8,
            >({
                let bit_3: u32 = unsafe { ::std::mem::transmute(bit_3) };
                bit_3 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                3usize,
                1u8,
            >({
                let bit_4: u32 = unsafe { ::std::mem::transmute(bit_4) };
                bit_4 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                4usize,
                1u8,
            >({
                let bit_5: u32 = unsafe { ::std::mem::transmute(bit_5) };
                bit_5 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                5usize,
                1u8,
            >({
                let bit_6: u32 = unsafe { ::std::mem::transmute(bit_6) };
                bit_6 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                6usize,
                1u8,
            >({
                let bit_7: u32 = unsafe { ::std::mem::transmute(bit_7) };
                bit_7 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                7usize,
                1u8,
            >({
                let bit_8: u32 = unsafe { ::std::mem::transmute(bit_8) };
                bit_8 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                8usize,
                1u8,
            >({
                let bit_9: u32 = unsafe { ::std::mem::transmute(bit_9) };
                bit_9 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                9usize,
                1u8,
            >({
                let bit_10: u32 = unsafe { ::std::mem::transmute(bit_10) };
                bit_10 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                10usize,
                1u8,
            >({
                let bit_11: u32 = unsafe { ::std::mem::transmute(bit_11) };
                bit_11 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                11usize,
                1u8,
            >({
                let bit_12: u32 = unsafe { ::std::mem::transmute(bit_12) };
                bit_12 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                12usize,
                1u8,
            >({
                let bit_13: u32 = unsafe { ::std::mem::transmute(bit_13) };
                bit_13 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                13usize,
                1u8,
            >({
                let bit_14: u32 = unsafe { ::std::mem::transmute(bit_14) };
                bit_14 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                14usize,
                1u8,
            >({
                let bit_15: u32 = unsafe { ::std::mem::transmute(bit_15) };
                bit_15 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                15usize,
                1u8,
            >({
                let bit_16: u32 = unsafe { ::std::mem::transmute(bit_16) };
                bit_16 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                16usize,
                1u8,
            >({
                let bit_17: u32 = unsafe { ::std::mem::transmute(bit_17) };
                bit_17 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                17usize,
                1u8,
            >({
                let bit_18: u32 = unsafe { ::std::mem::transmute(bit_18) };
                bit_18 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                18usize,
                1u8,
            >({
                let bit_19: u32 = unsafe { ::std::mem::transmute(bit_19) };
                bit_19 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                19usize,
                1u8,
            >({
                let bit_20: u32 = unsafe { ::std::mem::transmute(bit_20) };
                bit_20 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                20usize,
                1u8,
            >({
                let bit_21: u32 = unsafe { ::std::mem::transmute(bit_21) };
                bit_21 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                21usize,
                1u8,
            >({
                let bit_22: u32 = unsafe { ::std::mem::transmute(bit_22) };
                bit_22 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                22usize,
                1u8,
            >({
                let bit_23: u32 = unsafe { ::std::mem::transmute(bit_23) };
                bit_23 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                23usize,
                1u8,
            >({
                let bit_24: u32 = unsafe { ::std::mem::transmute(bit_24) };
                bit_24 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                24usize,
                1u8,
            >({
                let bit_25: u32 = unsafe { ::std::mem::transmute(bit_25) };
                bit_25 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                25usize,
                1u8,
            >({
                let bit_26: u32 = unsafe { ::std::mem::transmute(bit_26) };
                bit_26 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                26usize,
                1u8,
            >({
                let bit_27: u32 = unsafe { ::std::mem::transmute(bit_27) };
                bit_27 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                27usize,
                1u8,
            >({
                let bit_28: u32 = unsafe { ::std::mem::transmute(bit_28) };
                bit_28 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                28usize,
                1u8,
            >({
                let bit_29: u32 = unsafe { ::std::mem::transmute(bit_29) };
                bit_29 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                29usize,
                1u8,
            >({
                let bit_30: u32 = unsafe { ::std::mem::transmute(bit_30) };
                bit_30 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                30usize,
                1u8,
            >({
                let bit_31: u32 = unsafe { ::std::mem::transmute(bit_31) };
                bit_31 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                31usize,
                1u8,
            >({
                let bit_32: u32 = unsafe { ::std::mem::transmute(bit_32) };
                bit_32 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                32usize,
                1u8,
            >({
                let bit_33: u32 = unsafe { ::std::mem::transmute(bit_33) };
                bit_33 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                33usize,
                1u8,
            >({
                let bit_34: u32 = unsafe { ::std::mem::transmute(bit_34) };
                bit_34 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                34usize,
                1u8,
            >({
                let bit_35: u32 = unsafe { ::std::mem::transmute(bit_35) };
                bit_35 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                35usize,
                1u8,
            >({
                let bit_36: u32 = unsafe { ::std::mem::transmute(bit_36) };
                bit_36 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                36usize,
                1u8,
            >({
                let bit_37: u32 = unsafe { ::std::mem::transmute(bit_37) };
                bit_37 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                37usize,
                1u8,
            >({
                let bit_38: u32 = unsafe { ::std::mem::transmute(bit_38) };
                bit_38 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                38usize,
                1u8,
            >({
                let bit_39: u32 = unsafe { ::std::mem::transmute(bit_39) };
                bit_39 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                39usize,
                1u8,
            >({
                let bit_40: u32 = unsafe { ::std::mem::transmute(bit_40) };
                bit_40 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                40usize,
                1u8,
            >({
                let bit_41: u32 = unsafe { ::std::mem::transmute(bit_41) };
                bit_41 as u64
            });
        __bindgen_bitfield_unit
    }
}
