#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
#[allow(unused_qualifications)]
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
#[allow(unused_qualifications)]
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
    #[allow(dead_code)]
    pub unsafe fn raw_get_bit(this: *const Self, index: usize) -> bool {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = unsafe {
            *core::ptr::addr_of!((*this).storage).cast::<u8>().add(byte_index)
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
    #[allow(dead_code)]
    pub unsafe fn raw_set_bit(this: *mut Self, index: usize, val: bool) {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = unsafe {
            core::ptr::addr_of_mut!((*this).storage).cast::<u8>().add(byte_index)
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
            unsafe {
                Self::raw_set_bit(this, index + bit_offset, val_bit_is_set);
            };
        }
    }
}
/// <div rustbindgen derive="PartialEq,Eq"></div>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct first_bitfield_struct {
    pub _bindgen_align: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub __bindgen_padding_0: [u8; 3usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of first_bitfield_struct",
    ][::std::mem::size_of::<first_bitfield_struct>() - 4usize];
    [
        "Alignment of first_bitfield_struct",
    ][::std::mem::align_of::<first_bitfield_struct>() - 4usize];
};
impl first_bitfield_struct {
    #[inline]
    pub fn a(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_a(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn a_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 1usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 0usize, 1u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_a_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 1usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn b(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_b(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn b_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 1usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 1usize, 2u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_b_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 1usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                2u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn c(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 3u8) as u32) }
    }
    #[inline]
    pub fn set_c(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn c_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 1usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 3usize, 3u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_c_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 1usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                3usize,
                3u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        a: ::std::os::raw::c_uint,
        b: ::std::os::raw::c_uint,
        c: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit = __BindgenBitfieldUnit::new([0u8; 1usize]);
        __bindgen_bitfield_unit
            .set(
                0usize,
                1u8,
                {
                    let a: u32 = unsafe { ::std::mem::transmute(a) };
                    a as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                1usize,
                2u8,
                {
                    let b: u32 = unsafe { ::std::mem::transmute(b) };
                    b as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                3usize,
                3u8,
                {
                    let c: u32 = unsafe { ::std::mem::transmute(c) };
                    c as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
/// <div rustbindgen derive="Clone, Copy"></div>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct second_bitfield_struct {
    pub _bindgen_align: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub __bindgen_padding_0: [u8; 3usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of second_bitfield_struct",
    ][::std::mem::size_of::<second_bitfield_struct>() - 4usize];
    [
        "Alignment of second_bitfield_struct",
    ][::std::mem::align_of::<second_bitfield_struct>() - 4usize];
};
impl second_bitfield_struct {
    #[inline]
    pub fn x(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u32) }
    }
    #[inline]
    pub fn set_x(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn x_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 1usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 0usize, 4u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_x_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 1usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                4u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn y(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u32) }
    }
    #[inline]
    pub fn set_y(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn y_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 1usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 4usize, 4u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_y_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 1usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                4usize,
                4u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        x: ::std::os::raw::c_uint,
        y: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit = __BindgenBitfieldUnit::new([0u8; 1usize]);
        __bindgen_bitfield_unit
            .set(
                0usize,
                4u8,
                {
                    let x: u32 = unsafe { ::std::mem::transmute(x) };
                    x as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                4usize,
                4u8,
                {
                    let y: u32 = unsafe { ::std::mem::transmute(y) };
                    y as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
