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
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum nsStyleSVGOpacitySource {
    eStyleSVGOpacitySource_Normal = 0,
    eStyleSVGOpacitySource_ContextFillOpacity = 1,
    eStyleSVGOpacitySource_ContextStrokeOpacity = 2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Weird {
    pub mStrokeDasharrayLength: ::std::os::raw::c_uint,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    pub mClipRule: ::std::os::raw::c_uchar,
    pub mColorInterpolation: ::std::os::raw::c_uchar,
    pub mColorInterpolationFilters: ::std::os::raw::c_uchar,
    pub mFillRule: ::std::os::raw::c_uchar,
    pub mImageRendering: ::std::os::raw::c_uchar,
    pub mPaintOrder: ::std::os::raw::c_uchar,
    pub mShapeRendering: ::std::os::raw::c_uchar,
    pub mStrokeLinecap: ::std::os::raw::c_uchar,
    pub mStrokeLinejoin: ::std::os::raw::c_uchar,
    pub mTextAnchor: ::std::os::raw::c_uchar,
    pub mTextRendering: ::std::os::raw::c_uchar,
    pub _bitfield_2: __BindgenBitfieldUnit<[u8; 2usize]>,
    pub __bindgen_padding_0: [u8; 3usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Weird"][::std::mem::size_of::<Weird>() - 24usize];
    ["Alignment of Weird"][::std::mem::align_of::<Weird>() - 4usize];
    [
        "Offset of field: Weird::mStrokeDasharrayLength",
    ][::std::mem::offset_of!(Weird, mStrokeDasharrayLength) - 0usize];
    [
        "Offset of field: Weird::mClipRule",
    ][::std::mem::offset_of!(Weird, mClipRule) - 8usize];
    [
        "Offset of field: Weird::mColorInterpolation",
    ][::std::mem::offset_of!(Weird, mColorInterpolation) - 9usize];
    [
        "Offset of field: Weird::mColorInterpolationFilters",
    ][::std::mem::offset_of!(Weird, mColorInterpolationFilters) - 10usize];
    [
        "Offset of field: Weird::mFillRule",
    ][::std::mem::offset_of!(Weird, mFillRule) - 11usize];
    [
        "Offset of field: Weird::mImageRendering",
    ][::std::mem::offset_of!(Weird, mImageRendering) - 12usize];
    [
        "Offset of field: Weird::mPaintOrder",
    ][::std::mem::offset_of!(Weird, mPaintOrder) - 13usize];
    [
        "Offset of field: Weird::mShapeRendering",
    ][::std::mem::offset_of!(Weird, mShapeRendering) - 14usize];
    [
        "Offset of field: Weird::mStrokeLinecap",
    ][::std::mem::offset_of!(Weird, mStrokeLinecap) - 15usize];
    [
        "Offset of field: Weird::mStrokeLinejoin",
    ][::std::mem::offset_of!(Weird, mStrokeLinejoin) - 16usize];
    [
        "Offset of field: Weird::mTextAnchor",
    ][::std::mem::offset_of!(Weird, mTextAnchor) - 17usize];
    [
        "Offset of field: Weird::mTextRendering",
    ][::std::mem::offset_of!(Weird, mTextRendering) - 18usize];
};
impl Default for Weird {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Weird {
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn bitTest(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<0usize, 16u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_bitTest(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<0usize, 16u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn bitTest_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get_const::<
                    0usize,
                    16u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_bitTest_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                0usize,
                16u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn bitTest2(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get_const::<16usize, 15u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_bitTest2(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set_const::<16usize, 15u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn bitTest2_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get_const::<
                    16usize,
                    15u8,
                >(::std::ptr::addr_of!((*this)._bitfield_1)) as u32,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_bitTest2_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set_const::<
                16usize,
                15u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_1), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn new_bitfield_1(
        bitTest: ::std::os::raw::c_uint,
        bitTest2: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit
            .set_const::<
                0usize,
                16u8,
            >({
                let bitTest: u32 = unsafe { ::std::mem::transmute(bitTest) };
                bitTest as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                16usize,
                15u8,
            >({
                let bitTest2: u32 = unsafe { ::std::mem::transmute(bitTest2) };
                bitTest2 as u64
            });
        __bindgen_bitfield_unit
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn mFillOpacitySource(&self) -> nsStyleSVGOpacitySource {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get_const::<0usize, 3u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_mFillOpacitySource(&mut self, val: nsStyleSVGOpacitySource) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_2.set_const::<0usize, 3u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn mFillOpacitySource_raw(this: *const Self) -> nsStyleSVGOpacitySource {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    0usize,
                    3u8,
                >(::std::ptr::addr_of!((*this)._bitfield_2)) as u32,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_mFillOpacitySource_raw(
        this: *mut Self,
        val: nsStyleSVGOpacitySource,
    ) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                0usize,
                3u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_2), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn mStrokeOpacitySource(&self) -> nsStyleSVGOpacitySource {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get_const::<3usize, 3u8>() as u32)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_mStrokeOpacitySource(&mut self, val: nsStyleSVGOpacitySource) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_2.set_const::<3usize, 3u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn mStrokeOpacitySource_raw(
        this: *const Self,
    ) -> nsStyleSVGOpacitySource {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    3usize,
                    3u8,
                >(::std::ptr::addr_of!((*this)._bitfield_2)) as u32,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_mStrokeOpacitySource_raw(
        this: *mut Self,
        val: nsStyleSVGOpacitySource,
    ) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                3usize,
                3u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_2), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn mStrokeDasharrayFromObject(&self) -> bool {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get_const::<6usize, 1u8>() as u8)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_mStrokeDasharrayFromObject(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_2.set_const::<6usize, 1u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn mStrokeDasharrayFromObject_raw(this: *const Self) -> bool {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    6usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_2)) as u8,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_mStrokeDasharrayFromObject_raw(this: *mut Self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                6usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_2), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn mStrokeDashoffsetFromObject(&self) -> bool {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get_const::<7usize, 1u8>() as u8)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_mStrokeDashoffsetFromObject(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_2.set_const::<7usize, 1u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn mStrokeDashoffsetFromObject_raw(this: *const Self) -> bool {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    7usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_2)) as u8,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_mStrokeDashoffsetFromObject_raw(this: *mut Self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                7usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_2), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn mStrokeWidthFromObject(&self) -> bool {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get_const::<8usize, 1u8>() as u8)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn set_mStrokeWidthFromObject(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_2.set_const::<8usize, 1u8>(val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn mStrokeWidthFromObject_raw(this: *const Self) -> bool {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get_const::<
                    8usize,
                    1u8,
                >(::std::ptr::addr_of!((*this)._bitfield_2)) as u8,
            )
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub unsafe fn set_mStrokeWidthFromObject_raw(this: *mut Self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set_const::<
                8usize,
                1u8,
            >(::std::ptr::addr_of_mut!((*this)._bitfield_2), val as u64)
        }
    }
    #[inline]
    #[allow(unnecessary_transmutes)]
    pub fn new_bitfield_2(
        mFillOpacitySource: nsStyleSVGOpacitySource,
        mStrokeOpacitySource: nsStyleSVGOpacitySource,
        mStrokeDasharrayFromObject: bool,
        mStrokeDashoffsetFromObject: bool,
        mStrokeWidthFromObject: bool,
    ) -> __BindgenBitfieldUnit<[u8; 2usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize]> = Default::default();
        __bindgen_bitfield_unit
            .set_const::<
                0usize,
                3u8,
            >({
                let mFillOpacitySource: u32 = unsafe {
                    ::std::mem::transmute(mFillOpacitySource)
                };
                mFillOpacitySource as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                3usize,
                3u8,
            >({
                let mStrokeOpacitySource: u32 = unsafe {
                    ::std::mem::transmute(mStrokeOpacitySource)
                };
                mStrokeOpacitySource as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                6usize,
                1u8,
            >({
                let mStrokeDasharrayFromObject: u8 = unsafe {
                    ::std::mem::transmute(mStrokeDasharrayFromObject)
                };
                mStrokeDasharrayFromObject as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                7usize,
                1u8,
            >({
                let mStrokeDashoffsetFromObject: u8 = unsafe {
                    ::std::mem::transmute(mStrokeDashoffsetFromObject)
                };
                mStrokeDashoffsetFromObject as u64
            });
        __bindgen_bitfield_unit
            .set_const::<
                8usize,
                1u8,
            >({
                let mStrokeWidthFromObject: u8 = unsafe {
                    ::std::mem::transmute(mStrokeWidthFromObject)
                };
                mStrokeWidthFromObject as u64
            });
        __bindgen_bitfield_unit
    }
}
