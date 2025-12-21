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
#[derive(Debug, Copy, Clone)]
pub struct timex {
    pub tai: ::std::os::raw::c_int,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 44usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of timex"][::std::mem::size_of::<timex>() - 48usize];
    ["Alignment of timex"][::std::mem::align_of::<timex>() - 4usize];
    ["Offset of field: timex::tai"][::std::mem::offset_of!(timex, tai) - 0usize];
};
impl Default for timex {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timex_named {
    pub tai: ::std::os::raw::c_int,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 44usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of timex_named"][::std::mem::size_of::<timex_named>() - 48usize];
    ["Alignment of timex_named"][::std::mem::align_of::<timex_named>() - 4usize];
    [
        "Offset of field: timex_named::tai",
    ][::std::mem::offset_of!(timex_named, tai) - 0usize];
};
impl Default for timex_named {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl timex_named {
    #[inline]
    pub fn a(&self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<0usize, 32u8>() as u32)
        }
    }
    #[inline]
    pub fn set_a(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = val as _;
            self._bitfield_1.set_const::<0usize, 32u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn a_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 44usize],
                >>::raw_get_const::<
                    0usize,
                    32u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_a_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 44usize],
            >>::raw_set_const::<
                0usize,
                32u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn b(&self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<32usize, 32u8>() as u32)
        }
    }
    #[inline]
    pub fn set_b(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = val as _;
            self._bitfield_1.set_const::<32usize, 32u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn b_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 44usize],
                >>::raw_get_const::<
                    32usize,
                    32u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_b_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 44usize],
            >>::raw_set_const::<
                32usize,
                32u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn c(&self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<64usize, 32u8>() as u32)
        }
    }
    #[inline]
    pub fn set_c(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = val as _;
            self._bitfield_1.set_const::<64usize, 32u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn c_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 44usize],
                >>::raw_get_const::<
                    64usize,
                    32u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_c_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 44usize],
            >>::raw_set_const::<
                64usize,
                32u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn d(&self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<96usize, 32u8>() as u32)
        }
    }
    #[inline]
    pub fn set_d(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = val as _;
            self._bitfield_1.set_const::<96usize, 32u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn d_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 44usize],
                >>::raw_get_const::<
                    96usize,
                    32u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_d_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 44usize],
            >>::raw_set_const::<
                96usize,
                32u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn e(&self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<128usize, 32u8>() as u32)
        }
    }
    #[inline]
    pub fn set_e(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = val as _;
            self._bitfield_1.set_const::<128usize, 32u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn e_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 44usize],
                >>::raw_get_const::<
                    128usize,
                    32u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_e_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 44usize],
            >>::raw_set_const::<
                128usize,
                32u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn f(&self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<160usize, 32u8>() as u32)
        }
    }
    #[inline]
    pub fn set_f(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = val as _;
            self._bitfield_1.set_const::<160usize, 32u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn f_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 44usize],
                >>::raw_get_const::<
                    160usize,
                    32u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_f_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 44usize],
            >>::raw_set_const::<
                160usize,
                32u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn g(&self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<192usize, 32u8>() as u32)
        }
    }
    #[inline]
    pub fn set_g(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = val as _;
            self._bitfield_1.set_const::<192usize, 32u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn g_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 44usize],
                >>::raw_get_const::<
                    192usize,
                    32u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_g_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 44usize],
            >>::raw_set_const::<
                192usize,
                32u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn h(&self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<224usize, 32u8>() as u32)
        }
    }
    #[inline]
    pub fn set_h(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = val as _;
            self._bitfield_1.set_const::<224usize, 32u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn h_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 44usize],
                >>::raw_get_const::<
                    224usize,
                    32u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_h_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 44usize],
            >>::raw_set_const::<
                224usize,
                32u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn i(&self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<256usize, 32u8>() as u32)
        }
    }
    #[inline]
    pub fn set_i(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = val as _;
            self._bitfield_1.set_const::<256usize, 32u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn i_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 44usize],
                >>::raw_get_const::<
                    256usize,
                    32u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_i_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 44usize],
            >>::raw_set_const::<
                256usize,
                32u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn j(&self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<288usize, 32u8>() as u32)
        }
    }
    #[inline]
    pub fn set_j(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = val as _;
            self._bitfield_1.set_const::<288usize, 32u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn j_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 44usize],
                >>::raw_get_const::<
                    288usize,
                    32u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_j_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 44usize],
            >>::raw_set_const::<
                288usize,
                32u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    pub fn k(&self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<320usize, 32u8>() as u32)
        }
    }
    #[inline]
    pub fn set_k(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = val as _;
            self._bitfield_1.set_const::<320usize, 32u8>(val as u64)
        }
    }
    #[inline]
    pub unsafe fn k_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 44usize],
                >>::raw_get_const::<
                    320usize,
                    32u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_k_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = val as _;
            <__BindgenBitfieldUnit<
                [u8; 44usize],
            >>::raw_set_const::<
                320usize,
                32u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
}
