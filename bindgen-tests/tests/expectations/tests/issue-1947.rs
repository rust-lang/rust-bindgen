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
pub type U8 = ::std::os::raw::c_uchar;
pub type U16 = ::std::os::raw::c_ushort;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct V56AMDY {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize]>,
    pub MADK: U8,
    pub MABR: U8,
    pub _bitfield_align_2: [u16; 0],
    pub _bitfield_2: __BindgenBitfieldUnit<[u8; 3usize]>,
    pub _rB_: U8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of V56AMDY"][::std::mem::size_of::<V56AMDY>() - 8usize];
    ["Alignment of V56AMDY"][::std::mem::align_of::<V56AMDY>() - 2usize];
    ["Offset of field: V56AMDY::MADK"][::std::mem::offset_of!(V56AMDY, MADK) - 2usize];
    ["Offset of field: V56AMDY::MABR"][::std::mem::offset_of!(V56AMDY, MABR) - 3usize];
    ["Offset of field: V56AMDY::_rB_"][::std::mem::offset_of!(V56AMDY, _rB_) - 7usize];
};
impl V56AMDY {
    #[inline]
    pub fn MADZ(&self) -> U16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 10u8) as u16) }
    }
    #[inline]
    pub fn set_MADZ(&mut self, val: U16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 10u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn MADZ_raw(this: *const Self) -> U16 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 0usize, 10u8)
                    as u16,
            )
        }
    }
    #[inline]
    pub unsafe fn set_MADZ_raw(this: *mut Self, val: U16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                10u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn MAI0(&self) -> U16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(10usize, 2u8) as u16) }
    }
    #[inline]
    pub fn set_MAI0(&mut self, val: U16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(10usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn MAI0_raw(this: *const Self) -> U16 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 10usize, 2u8)
                    as u16,
            )
        }
    }
    #[inline]
    pub unsafe fn set_MAI0_raw(this: *mut Self, val: U16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                10usize,
                2u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn MAI1(&self) -> U16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(12usize, 2u8) as u16) }
    }
    #[inline]
    pub fn set_MAI1(&mut self, val: U16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(12usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn MAI1_raw(this: *const Self) -> U16 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 12usize, 2u8)
                    as u16,
            )
        }
    }
    #[inline]
    pub unsafe fn set_MAI1_raw(this: *mut Self, val: U16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                12usize,
                2u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn MAI2(&self) -> U16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(14usize, 2u8) as u16) }
    }
    #[inline]
    pub fn set_MAI2(&mut self, val: U16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(14usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn MAI2_raw(this: *const Self) -> U16 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 2usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 14usize, 2u8)
                    as u16,
            )
        }
    }
    #[inline]
    pub unsafe fn set_MAI2_raw(this: *mut Self, val: U16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 2usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                14usize,
                2u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        MADZ: U16,
        MAI0: U16,
        MAI1: U16,
        MAI2: U16,
    ) -> __BindgenBitfieldUnit<[u8; 2usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                10u8,
                {
                    let MADZ: u16 = unsafe { ::std::mem::transmute(MADZ) };
                    MADZ as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                10usize,
                2u8,
                {
                    let MAI0: u16 = unsafe { ::std::mem::transmute(MAI0) };
                    MAI0 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                12usize,
                2u8,
                {
                    let MAI1: u16 = unsafe { ::std::mem::transmute(MAI1) };
                    MAI1 as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                14usize,
                2u8,
                {
                    let MAI2: u16 = unsafe { ::std::mem::transmute(MAI2) };
                    MAI2 as u64
                },
            );
        __bindgen_bitfield_unit
    }
    #[inline]
    pub fn MATH(&self) -> U16 {
        unsafe { ::std::mem::transmute(self._bitfield_2.get(0usize, 10u8) as u16) }
    }
    #[inline]
    pub fn set_MATH(&mut self, val: U16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_2.set(0usize, 10u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn MATH_raw(this: *const Self) -> U16 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 3usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_2), 0usize, 10u8)
                    as u16,
            )
        }
    }
    #[inline]
    pub unsafe fn set_MATH_raw(this: *mut Self, val: U16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 3usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_2),
                0usize,
                10u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn MATE(&self) -> U16 {
        unsafe { ::std::mem::transmute(self._bitfield_2.get(10usize, 4u8) as u16) }
    }
    #[inline]
    pub fn set_MATE(&mut self, val: U16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_2.set(10usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn MATE_raw(this: *const Self) -> U16 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 3usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_2), 10usize, 4u8)
                    as u16,
            )
        }
    }
    #[inline]
    pub unsafe fn set_MATE_raw(this: *mut Self, val: U16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 3usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_2),
                10usize,
                4u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn MATW(&self) -> U16 {
        unsafe { ::std::mem::transmute(self._bitfield_2.get(14usize, 2u8) as u16) }
    }
    #[inline]
    pub fn set_MATW(&mut self, val: U16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_2.set(14usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn MATW_raw(this: *const Self) -> U16 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 3usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_2), 14usize, 2u8)
                    as u16,
            )
        }
    }
    #[inline]
    pub unsafe fn set_MATW_raw(this: *mut Self, val: U16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 3usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_2),
                14usize,
                2u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn MASW(&self) -> U8 {
        unsafe { ::std::mem::transmute(self._bitfield_2.get(16usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_MASW(&mut self, val: U8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_2.set(16usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn MASW_raw(this: *const Self) -> U8 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 3usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_2), 16usize, 4u8)
                    as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_MASW_raw(this: *mut Self, val: U8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 3usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_2),
                16usize,
                4u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn MABW(&self) -> U8 {
        unsafe { ::std::mem::transmute(self._bitfield_2.get(20usize, 3u8) as u8) }
    }
    #[inline]
    pub fn set_MABW(&mut self, val: U8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_2.set(20usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn MABW_raw(this: *const Self) -> U8 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 3usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_2), 20usize, 3u8)
                    as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_MABW_raw(this: *mut Self, val: U8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 3usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_2),
                20usize,
                3u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn MAXN(&self) -> U8 {
        unsafe { ::std::mem::transmute(self._bitfield_2.get(23usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_MAXN(&mut self, val: U8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_2.set(23usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn MAXN_raw(this: *const Self) -> U8 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 3usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_2), 23usize, 1u8)
                    as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_MAXN_raw(this: *mut Self, val: U8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 3usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_2),
                23usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_2(
        MATH: U16,
        MATE: U16,
        MATW: U16,
        MASW: U8,
        MABW: U8,
        MAXN: U8,
    ) -> __BindgenBitfieldUnit<[u8; 3usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 3usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                10u8,
                {
                    let MATH: u16 = unsafe { ::std::mem::transmute(MATH) };
                    MATH as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                10usize,
                4u8,
                {
                    let MATE: u16 = unsafe { ::std::mem::transmute(MATE) };
                    MATE as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                14usize,
                2u8,
                {
                    let MATW: u16 = unsafe { ::std::mem::transmute(MATW) };
                    MATW as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                16usize,
                4u8,
                {
                    let MASW: u8 = unsafe { ::std::mem::transmute(MASW) };
                    MASW as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                20usize,
                3u8,
                {
                    let MABW: u8 = unsafe { ::std::mem::transmute(MABW) };
                    MABW as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                23usize,
                1u8,
                {
                    let MAXN: u8 = unsafe { ::std::mem::transmute(MAXN) };
                    MAXN as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
