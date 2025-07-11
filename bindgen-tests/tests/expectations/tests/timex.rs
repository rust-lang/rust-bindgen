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
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 32u8) as u32) }
    }
    #[inline]
    pub fn set_a(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 32u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn a_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 44usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 0usize, 32u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_a_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 44usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                32u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn b(&self) -> ::std::os::raw::c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(32usize, 32u8) as u32) }
    }
    #[inline]
    pub fn set_b(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(32usize, 32u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn b_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 44usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 32usize, 32u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_b_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 44usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                32usize,
                32u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn c(&self) -> ::std::os::raw::c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(64usize, 32u8) as u32) }
    }
    #[inline]
    pub fn set_c(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(64usize, 32u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn c_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 44usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 64usize, 32u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_c_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 44usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                64usize,
                32u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn d(&self) -> ::std::os::raw::c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(96usize, 32u8) as u32) }
    }
    #[inline]
    pub fn set_d(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(96usize, 32u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn d_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 44usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 96usize, 32u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_d_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 44usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                96usize,
                32u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn e(&self) -> ::std::os::raw::c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(128usize, 32u8) as u32) }
    }
    #[inline]
    pub fn set_e(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(128usize, 32u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn e_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 44usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 128usize, 32u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_e_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 44usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                128usize,
                32u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn f(&self) -> ::std::os::raw::c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(160usize, 32u8) as u32) }
    }
    #[inline]
    pub fn set_f(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(160usize, 32u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn f_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 44usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 160usize, 32u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_f_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 44usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                160usize,
                32u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn g(&self) -> ::std::os::raw::c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(192usize, 32u8) as u32) }
    }
    #[inline]
    pub fn set_g(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(192usize, 32u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn g_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 44usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 192usize, 32u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_g_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 44usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                192usize,
                32u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn h(&self) -> ::std::os::raw::c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(224usize, 32u8) as u32) }
    }
    #[inline]
    pub fn set_h(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(224usize, 32u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn h_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 44usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 224usize, 32u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_h_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 44usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                224usize,
                32u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn i(&self) -> ::std::os::raw::c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(256usize, 32u8) as u32) }
    }
    #[inline]
    pub fn set_i(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(256usize, 32u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn i_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 44usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 256usize, 32u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_i_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 44usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                256usize,
                32u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn j(&self) -> ::std::os::raw::c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(288usize, 32u8) as u32) }
    }
    #[inline]
    pub fn set_j(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(288usize, 32u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn j_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 44usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 288usize, 32u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_j_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 44usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                288usize,
                32u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn k(&self) -> ::std::os::raw::c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(320usize, 32u8) as u32) }
    }
    #[inline]
    pub fn set_k(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(320usize, 32u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn k_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 44usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 320usize, 32u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_k_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 44usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                320usize,
                32u8,
                val as u64,
            )
        }
    }
}
