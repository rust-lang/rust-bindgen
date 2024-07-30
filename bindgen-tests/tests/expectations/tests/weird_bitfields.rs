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
        let byte = *(core::ptr::addr_of!((*this).storage) as *const u8)
            .offset(byte_index as isize);
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
        let byte = (core::ptr::addr_of_mut!((*this).storage) as *mut u8)
            .offset(byte_index as isize);
        *byte = Self::change_bit(*byte, index, val);
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
    pub unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>(),
        );
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if Self::raw_get_bit(this, i + bit_offset) {
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
    #[inline]
    pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>(),
        );
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            Self::raw_set_bit(this, index + bit_offset, val_bit_is_set);
        }
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
    pub _bitfield_align_1: [u16; 0],
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
    pub _bitfield_align_2: [u8; 0],
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
    pub fn bitTest(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_bitTest(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn bitTest_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 0usize, 16u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bitTest_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                16u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn bitTest2(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 15u8) as u32) }
    }
    #[inline]
    pub fn set_bitTest2(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 15u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn bitTest2_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 16usize, 15u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bitTest2_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                16usize,
                15u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        bitTest: ::std::os::raw::c_uint,
        bitTest2: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                16u8,
                {
                    let bitTest: u32 = unsafe { ::std::mem::transmute(bitTest) };
                    bitTest as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                16usize,
                15u8,
                {
                    let bitTest2: u32 = unsafe { ::std::mem::transmute(bitTest2) };
                    bitTest2 as u64
                },
            );
        __bindgen_bitfield_unit
    }
    #[inline]
    pub fn mFillOpacitySource(&self) -> nsStyleSVGOpacitySource {
        unsafe { ::std::mem::transmute(self._bitfield_2.get(0usize, 3u8) as u32) }
    }
    #[inline]
    pub fn set_mFillOpacitySource(&mut self, val: nsStyleSVGOpacitySource) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_2.set(0usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mFillOpacitySource_raw(this: *const Self) -> nsStyleSVGOpacitySource {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_2), 0usize, 3u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_mFillOpacitySource_raw(
        this: *mut Self,
        val: nsStyleSVGOpacitySource,
    ) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_2),
                0usize,
                3u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mStrokeOpacitySource(&self) -> nsStyleSVGOpacitySource {
        unsafe { ::std::mem::transmute(self._bitfield_2.get(3usize, 3u8) as u32) }
    }
    #[inline]
    pub fn set_mStrokeOpacitySource(&mut self, val: nsStyleSVGOpacitySource) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_2.set(3usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mStrokeOpacitySource_raw(
        this: *const Self,
    ) -> nsStyleSVGOpacitySource {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_2), 3usize, 3u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_mStrokeOpacitySource_raw(
        this: *mut Self,
        val: nsStyleSVGOpacitySource,
    ) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_2),
                3usize,
                3u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mStrokeDasharrayFromObject(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_2.get(6usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_mStrokeDasharrayFromObject(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_2.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mStrokeDasharrayFromObject_raw(this: *const Self) -> bool {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_2), 6usize, 1u8) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_mStrokeDasharrayFromObject_raw(this: *mut Self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_2),
                6usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mStrokeDashoffsetFromObject(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_2.get(7usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_mStrokeDashoffsetFromObject(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_2.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mStrokeDashoffsetFromObject_raw(this: *const Self) -> bool {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_2), 7usize, 1u8) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_mStrokeDashoffsetFromObject_raw(this: *mut Self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_2),
                7usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mStrokeWidthFromObject(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_2.get(8usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_mStrokeWidthFromObject(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_2.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mStrokeWidthFromObject_raw(this: *const Self) -> bool {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_2), 8usize, 1u8) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_mStrokeWidthFromObject_raw(this: *mut Self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_2),
                8usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_2(
        mFillOpacitySource: nsStyleSVGOpacitySource,
        mStrokeOpacitySource: nsStyleSVGOpacitySource,
        mStrokeDasharrayFromObject: bool,
        mStrokeDashoffsetFromObject: bool,
        mStrokeWidthFromObject: bool,
    ) -> __BindgenBitfieldUnit<[u8; 2usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                3u8,
                {
                    let mFillOpacitySource: u32 = unsafe {
                        ::std::mem::transmute(mFillOpacitySource)
                    };
                    mFillOpacitySource as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                3usize,
                3u8,
                {
                    let mStrokeOpacitySource: u32 = unsafe {
                        ::std::mem::transmute(mStrokeOpacitySource)
                    };
                    mStrokeOpacitySource as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                6usize,
                1u8,
                {
                    let mStrokeDasharrayFromObject: u8 = unsafe {
                        ::std::mem::transmute(mStrokeDasharrayFromObject)
                    };
                    mStrokeDasharrayFromObject as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                7usize,
                1u8,
                {
                    let mStrokeDashoffsetFromObject: u8 = unsafe {
                        ::std::mem::transmute(mStrokeDashoffsetFromObject)
                    };
                    mStrokeDashoffsetFromObject as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                8usize,
                1u8,
                {
                    let mStrokeWidthFromObject: u8 = unsafe {
                        ::std::mem::transmute(mStrokeWidthFromObject)
                    };
                    mStrokeWidthFromObject as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
