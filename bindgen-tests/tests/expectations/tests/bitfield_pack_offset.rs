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
            if unsafe { Self::raw_get_bit(this, i + bit_offset) } {
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
            unsafe { Self::raw_set_bit(this, index + bit_offset, val_bit_is_set) };
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct A {
    pub name: [::std::os::raw::c_uchar; 7usize],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 3usize]>,
    pub maxYield: ::std::os::raw::c_uchar,
    pub _bitfield_2: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub description1: *const ::std::os::raw::c_uchar,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of A"][::std::mem::size_of::<A>() - 24usize];
    ["Alignment of A"][::std::mem::align_of::<A>() - 8usize];
    ["Offset of field: A::name"][::std::mem::offset_of!(A, name) - 0usize];
    ["Offset of field: A::maxYield"][::std::mem::offset_of!(A, maxYield) - 10usize];
    [
        "Offset of field: A::description1",
    ][::std::mem::offset_of!(A, description1) - 16usize];
};
impl Default for A {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl A {
    #[inline]
    pub fn firmness(&self) -> ::std::os::raw::c_uchar {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_firmness(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn firmness_raw(this: *const Self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 3usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 0usize, 4u8) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_firmness_raw(this: *mut Self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 3usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                4u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn color(&self) -> ::std::os::raw::c_uchar {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_color(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn color_raw(this: *const Self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 3usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 4usize, 4u8) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_color_raw(this: *mut Self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 3usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                4usize,
                4u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn weedsBonus(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 3u8) as u16) }
    }
    #[inline]
    pub fn set_weedsBonus(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn weedsBonus_raw(this: *const Self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 3usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 8usize, 3u8)
                    as u16,
            )
        }
    }
    #[inline]
    pub unsafe fn set_weedsBonus_raw(this: *mut Self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 3usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                8usize,
                3u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn pestsBonus(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(11usize, 3u8) as u16) }
    }
    #[inline]
    pub fn set_pestsBonus(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(11usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn pestsBonus_raw(this: *const Self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 3usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 11usize, 3u8)
                    as u16,
            )
        }
    }
    #[inline]
    pub unsafe fn set_pestsBonus_raw(this: *mut Self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 3usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                11usize,
                3u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn size(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(14usize, 10u8) as u16) }
    }
    #[inline]
    pub fn set_size(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(14usize, 10u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn size_raw(this: *const Self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 3usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 14usize, 10u8)
                    as u16,
            )
        }
    }
    #[inline]
    pub unsafe fn set_size_raw(this: *mut Self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 3usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                14usize,
                10u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        firmness: ::std::os::raw::c_uchar,
        color: ::std::os::raw::c_uchar,
        weedsBonus: ::std::os::raw::c_ushort,
        pestsBonus: ::std::os::raw::c_ushort,
        size: ::std::os::raw::c_ushort,
    ) -> __BindgenBitfieldUnit<[u8; 3usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 3usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                4u8,
                {
                    let firmness: u8 = unsafe { ::std::mem::transmute(firmness) };
                    firmness as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                4usize,
                4u8,
                {
                    let color: u8 = unsafe { ::std::mem::transmute(color) };
                    color as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                8usize,
                3u8,
                {
                    let weedsBonus: u16 = unsafe { ::std::mem::transmute(weedsBonus) };
                    weedsBonus as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                11usize,
                3u8,
                {
                    let pestsBonus: u16 = unsafe { ::std::mem::transmute(pestsBonus) };
                    pestsBonus as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                14usize,
                10u8,
                {
                    let size: u16 = unsafe { ::std::mem::transmute(size) };
                    size as u64
                },
            );
        __bindgen_bitfield_unit
    }
    #[inline]
    pub fn minYield(&self) -> ::std::os::raw::c_uchar {
        unsafe { ::std::mem::transmute(self._bitfield_2.get(0usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_minYield(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_2.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn minYield_raw(this: *const Self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 1usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_2), 0usize, 4u8) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_minYield_raw(this: *mut Self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 1usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_2),
                0usize,
                4u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn waterBonus(&self) -> ::std::os::raw::c_uchar {
        unsafe { ::std::mem::transmute(self._bitfield_2.get(4usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_waterBonus(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_2.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn waterBonus_raw(this: *const Self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 1usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_2), 4usize, 4u8) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_waterBonus_raw(this: *mut Self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 1usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_2),
                4usize,
                4u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_2(
        minYield: ::std::os::raw::c_uchar,
        waterBonus: ::std::os::raw::c_uchar,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                4u8,
                {
                    let minYield: u8 = unsafe { ::std::mem::transmute(minYield) };
                    minYield as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                4usize,
                4u8,
                {
                    let waterBonus: u8 = unsafe { ::std::mem::transmute(waterBonus) };
                    waterBonus as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
