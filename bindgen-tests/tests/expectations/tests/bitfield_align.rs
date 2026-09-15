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
        let Self { storage } = self;
        let storage = storage.as_ref();
        unsafe { Self::get_bits(storage.as_ptr(), storage.len(), bit_offset, bit_width) }
    }
    #[inline]
    pub unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64 {
        unsafe {
            Self::get_bits(
                core::ptr::addr_of!((*this).storage).cast(),
                core::mem::size_of::<Storage>(),
                bit_offset,
                bit_width,
            )
        }
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        let Self { storage } = self;
        let storage = storage.as_mut();
        unsafe {
            Self::set_bits(
                storage.as_mut_ptr(),
                storage.len(),
                bit_offset,
                bit_width,
                val,
            );
        }
    }
    #[inline]
    pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) {
        unsafe {
            Self::set_bits(
                core::ptr::addr_of_mut!((*this).storage).cast(),
                core::mem::size_of::<Storage>(),
                bit_offset,
                bit_width,
                val,
            );
        }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    const fn field_layout(
        storage_len: usize,
        bit_offset: usize,
        bit_width: u8,
    ) -> (usize, usize, usize) {
        debug_assert!(bit_width <= 64);
        let start_byte = bit_offset / 8;
        let bit_shift = bit_offset % 8;
        debug_assert!(start_byte < storage_len);
        let bytes_needed = (bit_width as usize + bit_shift + 7) / 8;
        assert!(start_byte + bytes_needed <= storage_len);
        (start_byte, bit_shift, bytes_needed)
    }
    #[inline]
    const unsafe fn get_bits(
        storage: *const u8,
        storage_len: usize,
        bit_offset: usize,
        bit_width: u8,
    ) -> u64 {
        let (start_byte, bit_shift, bytes_needed) = Self::field_layout(
            storage_len,
            bit_offset,
            bit_width,
        );
        if bit_width == 0 {
            return 0;
        }
        let mut val = 0u64;
        let mut i = 0;
        while i < bytes_needed && i < 8 {
            let byte = unsafe { *storage.add(start_byte + i) };
            let byte = if cfg!(target_endian = "big") {
                byte.reverse_bits()
            } else {
                byte
            };
            val |= (byte as u64) << (i * 8);
            i += 1;
        }
        val >>= bit_shift;
        if bytes_needed == 9 {
            let byte = unsafe { *storage.add(start_byte + 8) };
            let byte = if cfg!(target_endian = "big") {
                byte.reverse_bits()
            } else {
                byte
            };
            val |= (byte as u64) << (64 - bit_shift);
        }
        if bit_width < 64 {
            val &= (1u64 << bit_width) - 1;
        }
        if cfg!(target_endian = "big") {
            val = val.reverse_bits() >> (64 - bit_width as usize);
        }
        val
    }
    #[inline]
    unsafe fn set_bits(
        storage: *mut u8,
        storage_len: usize,
        bit_offset: usize,
        bit_width: u8,
        mut val: u64,
    ) {
        let (start_byte, bit_shift, bytes_needed) = Self::field_layout(
            storage_len,
            bit_offset,
            bit_width,
        );
        if bit_width == 0 {
            return;
        }
        if bit_width == 64 && bit_shift == 0 {
            unsafe { storage.add(start_byte).cast::<u64>().write_unaligned(val) };
            return;
        }
        let field_mask = if bit_width == 64 { !0u64 } else { (1u64 << bit_width) - 1 };
        val &= field_mask;
        if cfg!(target_endian = "big") {
            val = val.reverse_bits() >> (64 - bit_width as usize);
        }
        if bytes_needed == 9 {
            let shift = 64 - bit_shift;
            unsafe {
                Self::set_byte(
                    storage.add(start_byte + 8),
                    (val >> shift) as u8,
                    (field_mask >> shift) as u8,
                );
            }
        }
        let val = val << bit_shift;
        let field_mask = field_mask << bit_shift;
        for i in 0..bytes_needed.min(8) {
            unsafe {
                Self::set_byte(
                    storage.add(start_byte + i),
                    (val >> (i * 8)) as u8,
                    (field_mask >> (i * 8)) as u8,
                );
            }
        }
    }
    #[inline]
    unsafe fn set_byte(byte: *mut u8, val: u8, mask: u8) {
        let (val, mask) = if cfg!(target_endian = "big") {
            (val.reverse_bits(), mask.reverse_bits())
        } else {
            (val, mask)
        };
        unsafe { *byte = (*byte & !mask) | (val & mask) };
    }
}
/// Const-generic methods for bitfield access when offset and width are known
/// at compile time. Inlining exposes these constants to the shared implementation.
impl<const N: usize> __BindgenBitfieldUnit<[u8; N]> {
    /// Get a field using const generics for compile-time optimization.
    #[inline]
    pub const fn get_const<const BIT_OFFSET: usize, const BIT_WIDTH: u8>(&self) -> u64 {
        let Self { storage } = self;
        unsafe { Self::get_bits(storage.as_ptr(), N, BIT_OFFSET, BIT_WIDTH) }
    }
    /// Set a field using const generics for compile-time optimization.
    #[inline]
    pub fn set_const<const BIT_OFFSET: usize, const BIT_WIDTH: u8>(&mut self, val: u64) {
        self.set(BIT_OFFSET, BIT_WIDTH, val);
    }
    /// Raw pointer get using const generics for compile-time optimization.
    #[inline]
    pub const unsafe fn raw_get_const<const BIT_OFFSET: usize, const BIT_WIDTH: u8>(
        this: *const Self,
    ) -> u64 {
        unsafe {
            Self::get_bits(
                core::ptr::addr_of!((*this).storage).cast(),
                N,
                BIT_OFFSET,
                BIT_WIDTH,
            )
        }
    }
    /// Raw pointer set using const generics for compile-time optimization.
    #[inline]
    pub unsafe fn raw_set_const<const BIT_OFFSET: usize, const BIT_WIDTH: u8>(
        this: *mut Self,
        val: u64,
    ) {
        unsafe { Self::raw_set(this, BIT_OFFSET, BIT_WIDTH, val) };
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct A {
    pub _bindgen_align: [u32; 0],
    pub x: ::std::os::raw::c_uchar,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize]>,
    pub y: ::std::os::raw::c_uchar,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of A"][::std::mem::size_of::<A>() - 4usize];
    ["Alignment of A"][::std::mem::align_of::<A>() - 4usize];
    ["Offset of field: A::x"][::std::mem::offset_of!(A, x) - 0usize];
    ["Offset of field: A::y"][::std::mem::offset_of!(A, y) - 3usize];
};
impl A {
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn b1(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<0usize, 1u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_b1(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<0usize, 1u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn b1_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    0usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_b1_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                0usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn b2(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<1usize, 1u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_b2(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<1usize, 1u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn b2_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    1usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_b2_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                1usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn b3(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<2usize, 1u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_b3(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<2usize, 1u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn b3_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    2usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_b3_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                2usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn b4(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<3usize, 1u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_b4(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<3usize, 1u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn b4_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    3usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_b4_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                3usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn b5(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<4usize, 1u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_b5(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<4usize, 1u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn b5_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    4usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_b5_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                4usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn b6(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<5usize, 1u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_b6(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<5usize, 1u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn b6_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    5usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_b6_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                5usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn b7(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<6usize, 1u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_b7(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<6usize, 1u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn b7_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    6usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_b7_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                6usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn b8(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<7usize, 1u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_b8(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<7usize, 1u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn b8_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    7usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_b8_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                7usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn b9(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<8usize, 1u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_b9(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<8usize, 1u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn b9_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    8usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_b9_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                8usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn b10(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<9usize, 1u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_b10(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<9usize, 1u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn b10_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    9usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_b10_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                9usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn new_bitfield_1(
        b1: ::std::os::raw::c_uint,
        b2: ::std::os::raw::c_uint,
        b3: ::std::os::raw::c_uint,
        b4: ::std::os::raw::c_uint,
        b5: ::std::os::raw::c_uint,
        b6: ::std::os::raw::c_uint,
        b7: ::std::os::raw::c_uint,
        b8: ::std::os::raw::c_uint,
        b9: ::std::os::raw::c_uint,
        b10: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 2usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize]> = Default::default();
        __bindgen_bitfield_unit
            .set_const::<
                0usize,
                1u8,
            >({
                let b1: u32 = unsafe { ::std::mem::transmute(b1) };
                b1 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                1usize,
                1u8,
            >({
                let b2: u32 = unsafe { ::std::mem::transmute(b2) };
                b2 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                2usize,
                1u8,
            >({
                let b3: u32 = unsafe { ::std::mem::transmute(b3) };
                b3 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                3usize,
                1u8,
            >({
                let b4: u32 = unsafe { ::std::mem::transmute(b4) };
                b4 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                4usize,
                1u8,
            >({
                let b5: u32 = unsafe { ::std::mem::transmute(b5) };
                b5 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                5usize,
                1u8,
            >({
                let b6: u32 = unsafe { ::std::mem::transmute(b6) };
                b6 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                6usize,
                1u8,
            >({
                let b7: u32 = unsafe { ::std::mem::transmute(b7) };
                b7 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                7usize,
                1u8,
            >({
                let b8: u32 = unsafe { ::std::mem::transmute(b8) };
                b8 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                8usize,
                1u8,
            >({
                let b9: u32 = unsafe { ::std::mem::transmute(b9) };
                b9 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                9usize,
                1u8,
            >({
                let b10: u32 = unsafe { ::std::mem::transmute(b10) };
                b10 as u64
            });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct B {
    pub _bindgen_align: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of B"][::std::mem::size_of::<B>() - 4usize];
    ["Alignment of B"][::std::mem::align_of::<B>() - 4usize];
};
impl B {
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn foo(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<0usize, 31u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_foo(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<0usize, 31u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn foo_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get_const::<
                    0usize,
                    31u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_foo_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                0usize,
                31u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn bar(&self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<31usize, 1u8>() as u8)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_bar(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<31usize, 1u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn bar_raw(this: *const Self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get_const::<
                    31usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_bar_raw(this: *mut Self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                31usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn new_bitfield_1(
        foo: ::std::os::raw::c_uint,
        bar: ::std::os::raw::c_uchar,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit
            .set_const::<
                0usize,
                31u8,
            >({
                let foo: u32 = unsafe { ::std::mem::transmute(foo) };
                foo as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                31usize,
                1u8,
            >({
                let bar: u8 = unsafe { ::std::mem::transmute(bar) };
                bar as u64
            });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct C {
    pub x: ::std::os::raw::c_uchar,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub baz: ::std::os::raw::c_uint,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of C"][::std::mem::size_of::<C>() - 8usize];
    ["Alignment of C"][::std::mem::align_of::<C>() - 4usize];
    ["Offset of field: C::x"][::std::mem::offset_of!(C, x) - 0usize];
    ["Offset of field: C::baz"][::std::mem::offset_of!(C, baz) - 4usize];
};
impl C {
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn b1(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<0usize, 1u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_b1(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<0usize, 1u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn b1_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 1usize],
                >>::raw_get_const::<
                    0usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_b1_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 1usize],
            >>::raw_set_const::<
                0usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn b2(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<1usize, 1u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_b2(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<1usize, 1u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn b2_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 1usize],
                >>::raw_get_const::<
                    1usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_b2_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 1usize],
            >>::raw_set_const::<
                1usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn new_bitfield_1(
        b1: ::std::os::raw::c_uint,
        b2: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit
            .set_const::<
                0usize,
                1u8,
            >({
                let b1: u32 = unsafe { ::std::mem::transmute(b1) };
                b1 as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                1usize,
                1u8,
            >({
                let b2: u32 = unsafe { ::std::mem::transmute(b2) };
                b2 as u64
            });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Date1 {
    pub _bindgen_align: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 3usize]>,
    pub __bindgen_padding_0: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Date1"][::std::mem::size_of::<Date1>() - 4usize];
    ["Alignment of Date1"][::std::mem::align_of::<Date1>() - 2usize];
};
impl Date1 {
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn nWeekDay(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<0usize, 3u8>() as u16)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_nWeekDay(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<0usize, 3u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn nWeekDay_raw(this: *const Self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 3usize],
                >>::raw_get_const::<
                    0usize,
                    3u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u16,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_nWeekDay_raw(this: *mut Self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 3usize],
            >>::raw_set_const::<
                0usize,
                3u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn nMonthDay(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<3usize, 6u8>() as u16)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_nMonthDay(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<3usize, 6u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn nMonthDay_raw(this: *const Self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 3usize],
                >>::raw_get_const::<
                    3usize,
                    6u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u16,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_nMonthDay_raw(this: *mut Self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 3usize],
            >>::raw_set_const::<
                3usize,
                6u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn nMonth(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<9usize, 5u8>() as u16)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_nMonth(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<9usize, 5u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn nMonth_raw(this: *const Self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 3usize],
                >>::raw_get_const::<
                    9usize,
                    5u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u16,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_nMonth_raw(this: *mut Self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 3usize],
            >>::raw_set_const::<
                9usize,
                5u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn nYear(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<16usize, 8u8>() as u16)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_nYear(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<16usize, 8u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn nYear_raw(this: *const Self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 3usize],
                >>::raw_get_const::<
                    16usize,
                    8u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u16,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_nYear_raw(this: *mut Self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 3usize],
            >>::raw_set_const::<
                16usize,
                8u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn new_bitfield_1(
        nWeekDay: ::std::os::raw::c_ushort,
        nMonthDay: ::std::os::raw::c_ushort,
        nMonth: ::std::os::raw::c_ushort,
        nYear: ::std::os::raw::c_ushort,
    ) -> __BindgenBitfieldUnit<[u8; 3usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 3usize]> = Default::default();
        __bindgen_bitfield_unit
            .set_const::<
                0usize,
                3u8,
            >({
                let nWeekDay: u16 = unsafe { ::std::mem::transmute(nWeekDay) };
                nWeekDay as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                3usize,
                6u8,
            >({
                let nMonthDay: u16 = unsafe { ::std::mem::transmute(nMonthDay) };
                nMonthDay as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                9usize,
                5u8,
            >({
                let nMonth: u16 = unsafe { ::std::mem::transmute(nMonth) };
                nMonth as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                16usize,
                8u8,
            >({
                let nYear: u16 = unsafe { ::std::mem::transmute(nYear) };
                nYear as u64
            });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Date2 {
    pub _bindgen_align: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Date2"][::std::mem::size_of::<Date2>() - 4usize];
    ["Alignment of Date2"][::std::mem::align_of::<Date2>() - 2usize];
};
impl Date2 {
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn nWeekDay(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<0usize, 3u8>() as u16)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_nWeekDay(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<0usize, 3u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn nWeekDay_raw(this: *const Self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get_const::<
                    0usize,
                    3u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u16,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_nWeekDay_raw(this: *mut Self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                0usize,
                3u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn nMonthDay(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<3usize, 6u8>() as u16)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_nMonthDay(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<3usize, 6u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn nMonthDay_raw(this: *const Self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get_const::<
                    3usize,
                    6u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u16,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_nMonthDay_raw(this: *mut Self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                3usize,
                6u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn nMonth(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<9usize, 5u8>() as u16)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_nMonth(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<9usize, 5u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn nMonth_raw(this: *const Self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get_const::<
                    9usize,
                    5u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u16,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_nMonth_raw(this: *mut Self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                9usize,
                5u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn nYear(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<16usize, 8u8>() as u16)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_nYear(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<16usize, 8u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn nYear_raw(this: *const Self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get_const::<
                    16usize,
                    8u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u16,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_nYear_raw(this: *mut Self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                16usize,
                8u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn byte(&self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<24usize, 8u8>() as u8)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_byte(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<24usize, 8u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn byte_raw(this: *const Self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get_const::<
                    24usize,
                    8u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u8,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_byte_raw(this: *mut Self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                24usize,
                8u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn new_bitfield_1(
        nWeekDay: ::std::os::raw::c_ushort,
        nMonthDay: ::std::os::raw::c_ushort,
        nMonth: ::std::os::raw::c_ushort,
        nYear: ::std::os::raw::c_ushort,
        byte: ::std::os::raw::c_uchar,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit
            .set_const::<
                0usize,
                3u8,
            >({
                let nWeekDay: u16 = unsafe { ::std::mem::transmute(nWeekDay) };
                nWeekDay as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                3usize,
                6u8,
            >({
                let nMonthDay: u16 = unsafe { ::std::mem::transmute(nMonthDay) };
                nMonthDay as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                9usize,
                5u8,
            >({
                let nMonth: u16 = unsafe { ::std::mem::transmute(nMonth) };
                nMonth as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                16usize,
                8u8,
            >({
                let nYear: u16 = unsafe { ::std::mem::transmute(nYear) };
                nYear as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                24usize,
                8u8,
            >({
                let byte: u8 = unsafe { ::std::mem::transmute(byte) };
                byte as u64
            });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Date3 {
    pub _bindgen_align: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 3usize]>,
    pub byte: ::std::os::raw::c_uchar,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Date3"][::std::mem::size_of::<Date3>() - 4usize];
    ["Alignment of Date3"][::std::mem::align_of::<Date3>() - 2usize];
    ["Offset of field: Date3::byte"][::std::mem::offset_of!(Date3, byte) - 3usize];
};
impl Date3 {
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn nWeekDay(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<0usize, 3u8>() as u16)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_nWeekDay(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<0usize, 3u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn nWeekDay_raw(this: *const Self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 3usize],
                >>::raw_get_const::<
                    0usize,
                    3u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u16,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_nWeekDay_raw(this: *mut Self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 3usize],
            >>::raw_set_const::<
                0usize,
                3u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn nMonthDay(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<3usize, 6u8>() as u16)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_nMonthDay(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<3usize, 6u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn nMonthDay_raw(this: *const Self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 3usize],
                >>::raw_get_const::<
                    3usize,
                    6u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u16,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_nMonthDay_raw(this: *mut Self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 3usize],
            >>::raw_set_const::<
                3usize,
                6u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn nMonth(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<9usize, 5u8>() as u16)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_nMonth(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<9usize, 5u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn nMonth_raw(this: *const Self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 3usize],
                >>::raw_get_const::<
                    9usize,
                    5u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u16,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_nMonth_raw(this: *mut Self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 3usize],
            >>::raw_set_const::<
                9usize,
                5u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn nYear(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<16usize, 8u8>() as u16)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_nYear(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<16usize, 8u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn nYear_raw(this: *const Self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 3usize],
                >>::raw_get_const::<
                    16usize,
                    8u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u16,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_nYear_raw(this: *mut Self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 3usize],
            >>::raw_set_const::<
                16usize,
                8u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn new_bitfield_1(
        nWeekDay: ::std::os::raw::c_ushort,
        nMonthDay: ::std::os::raw::c_ushort,
        nMonth: ::std::os::raw::c_ushort,
        nYear: ::std::os::raw::c_ushort,
    ) -> __BindgenBitfieldUnit<[u8; 3usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 3usize]> = Default::default();
        __bindgen_bitfield_unit
            .set_const::<
                0usize,
                3u8,
            >({
                let nWeekDay: u16 = unsafe { ::std::mem::transmute(nWeekDay) };
                nWeekDay as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                3usize,
                6u8,
            >({
                let nMonthDay: u16 = unsafe { ::std::mem::transmute(nMonthDay) };
                nMonthDay as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                9usize,
                5u8,
            >({
                let nMonth: u16 = unsafe { ::std::mem::transmute(nMonth) };
                nMonth as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                16usize,
                8u8,
            >({
                let nYear: u16 = unsafe { ::std::mem::transmute(nYear) };
                nYear as u64
            });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Gap {
    pub _bindgen_align: [u32; 0],
    pub a: ::std::os::raw::c_char,
    pub __bindgen_padding_0: [u8; 3usize],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    pub c: ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Gap"][::std::mem::size_of::<Gap>() - 12usize];
    ["Alignment of Gap"][::std::mem::align_of::<Gap>() - 4usize];
    ["Offset of field: Gap::a"][::std::mem::offset_of!(Gap, a) - 0usize];
    ["Offset of field: Gap::c"][::std::mem::offset_of!(Gap, c) - 8usize];
};
impl Gap {
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn b(&self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<0usize, 30u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_b(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<0usize, 30u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn b_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get_const::<
                    0usize,
                    30u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_b_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                0usize,
                30u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn new_bitfield_1(
        b: ::std::os::raw::c_int,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit
            .set_const::<
                0usize,
                30u8,
            >({
                let b: u32 = unsafe { ::std::mem::transmute(b) };
                b as u64
            });
        __bindgen_bitfield_unit
    }
}
pub type U = ::std::os::raw::c_uint;
#[repr(C, packed(2))]
#[derive(Debug, Default, Copy, Clone)]
pub struct UnderAligned {
    pub before: ::std::os::raw::c_char,
    pub __bindgen_padding_0: u8,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    pub mid: ::std::os::raw::c_char,
    pub inner: U,
    pub tail: ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of UnderAligned"][::std::mem::size_of::<UnderAligned>() - 14usize];
    ["Alignment of UnderAligned"][::std::mem::align_of::<UnderAligned>() - 2usize];
    [
        "Offset of field: UnderAligned::before",
    ][::std::mem::offset_of!(UnderAligned, before) - 0usize];
    [
        "Offset of field: UnderAligned::mid",
    ][::std::mem::offset_of!(UnderAligned, mid) - 6usize];
    [
        "Offset of field: UnderAligned::inner",
    ][::std::mem::offset_of!(UnderAligned, inner) - 8usize];
    [
        "Offset of field: UnderAligned::tail",
    ][::std::mem::offset_of!(UnderAligned, tail) - 12usize];
};
impl UnderAligned {
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn bits(&self) -> U {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<0usize, 31u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_bits(&mut self, val: U) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<0usize, 31u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn bits_raw(this: *const Self) -> U {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get_const::<
                    0usize,
                    31u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_bits_raw(this: *mut Self, val: U) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                0usize,
                31u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn new_bitfield_1(bits: U) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit
            .set_const::<
                0usize,
                31u8,
            >({
                let bits: u32 = unsafe { ::std::mem::transmute(bits) };
                bits as u64
            });
        __bindgen_bitfield_unit
    }
}
