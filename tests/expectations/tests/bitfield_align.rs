#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

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
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <=
                self.storage.as_ref().len()
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
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <=
                self.storage.as_ref().len()
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
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct A {
    pub x: ::std::os::raw::c_uchar,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize]>,
    pub y: ::std::os::raw::c_uchar,
}
#[test]
fn bindgen_test_layout_A() {
    assert_eq!(
        ::std::mem::size_of::<A>(),
        4usize,
        concat!("Size of: ", stringify!(A))
    );
    assert_eq!(
        ::std::mem::align_of::<A>(),
        4usize,
        concat!("Alignment of ", stringify!(A))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A>())).x as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(A), "::", stringify!(x))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A>())).y as *const _ as usize },
        3usize,
        concat!("Offset of field: ", stringify!(A), "::", stringify!(y))
    );
}
impl A {
    #[inline]
    pub fn b1(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32)
        }
    }
    #[inline]
    pub fn set_b1(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn b2(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32)
        }
    }
    #[inline]
    pub fn set_b2(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn b3(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32)
        }
    }
    #[inline]
    pub fn set_b3(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn b4(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32)
        }
    }
    #[inline]
    pub fn set_b4(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn b5(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32)
        }
    }
    #[inline]
    pub fn set_b5(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn b6(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32)
        }
    }
    #[inline]
    pub fn set_b6(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn b7(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u32)
        }
    }
    #[inline]
    pub fn set_b7(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn b8(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32)
        }
    }
    #[inline]
    pub fn set_b8(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn b9(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u32)
        }
    }
    #[inline]
    pub fn set_b9(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn b10(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u32)
        }
    }
    #[inline]
    pub fn set_b10(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(9usize, 1u8, val as u64)
        }
    }
    #[inline]
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
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize]> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let b1: u32 = unsafe { ::std::mem::transmute(b1) };
            b1 as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let b2: u32 = unsafe { ::std::mem::transmute(b2) };
            b2 as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let b3: u32 = unsafe { ::std::mem::transmute(b3) };
            b3 as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let b4: u32 = unsafe { ::std::mem::transmute(b4) };
            b4 as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let b5: u32 = unsafe { ::std::mem::transmute(b5) };
            b5 as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let b6: u32 = unsafe { ::std::mem::transmute(b6) };
            b6 as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let b7: u32 = unsafe { ::std::mem::transmute(b7) };
            b7 as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let b8: u32 = unsafe { ::std::mem::transmute(b8) };
            b8 as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let b9: u32 = unsafe { ::std::mem::transmute(b9) };
            b9 as u64
        });
        __bindgen_bitfield_unit.set(9usize, 1u8, {
            let b10: u32 = unsafe { ::std::mem::transmute(b10) };
            b10 as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct B {
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[test]
fn bindgen_test_layout_B() {
    assert_eq!(
        ::std::mem::size_of::<B>(),
        4usize,
        concat!("Size of: ", stringify!(B))
    );
    assert_eq!(
        ::std::mem::align_of::<B>(),
        4usize,
        concat!("Alignment of ", stringify!(B))
    );
}
impl B {
    #[inline]
    pub fn foo(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 31u8) as u32)
        }
    }
    #[inline]
    pub fn set_foo(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 31u8, val as u64)
        }
    }
    #[inline]
    pub fn bar(&self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(31usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_bar(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(31usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        foo: ::std::os::raw::c_uint,
        bar: ::std::os::raw::c_uchar,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 31u8, {
            let foo: u32 = unsafe { ::std::mem::transmute(foo) };
            foo as u64
        });
        __bindgen_bitfield_unit.set(31usize, 1u8, {
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
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub baz: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_C() {
    assert_eq!(
        ::std::mem::size_of::<C>(),
        8usize,
        concat!("Size of: ", stringify!(C))
    );
    assert_eq!(
        ::std::mem::align_of::<C>(),
        4usize,
        concat!("Alignment of ", stringify!(C))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<C>())).x as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(C), "::", stringify!(x))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<C>())).baz as *const _ as usize },
        4usize,
        concat!("Offset of field: ", stringify!(C), "::", stringify!(baz))
    );
}
impl C {
    #[inline]
    pub fn b1(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32)
        }
    }
    #[inline]
    pub fn set_b1(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn b2(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32)
        }
    }
    #[inline]
    pub fn set_b2(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        b1: ::std::os::raw::c_uint,
        b2: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let b1: u32 = unsafe { ::std::mem::transmute(b1) };
            b1 as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let b2: u32 = unsafe { ::std::mem::transmute(b2) };
            b2 as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[repr(align(2))]
#[derive(Debug, Default, Copy, Clone)]
pub struct Date1 {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 3usize]>,
    pub __bindgen_padding_0: u8,
}
#[test]
fn bindgen_test_layout_Date1() {
    assert_eq!(
        ::std::mem::size_of::<Date1>(),
        4usize,
        concat!("Size of: ", stringify!(Date1))
    );
    assert_eq!(
        ::std::mem::align_of::<Date1>(),
        2usize,
        concat!("Alignment of ", stringify!(Date1))
    );
}
impl Date1 {
    #[inline]
    pub fn nWeekDay(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 3u8) as u16)
        }
    }
    #[inline]
    pub fn set_nWeekDay(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn nMonthDay(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(3usize, 6u8) as u16)
        }
    }
    #[inline]
    pub fn set_nMonthDay(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub fn nMonth(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(9usize, 5u8) as u16)
        }
    }
    #[inline]
    pub fn set_nMonth(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(9usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub fn nYear(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(16usize, 8u8) as u16)
        }
    }
    #[inline]
    pub fn set_nYear(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        nWeekDay: ::std::os::raw::c_ushort,
        nMonthDay: ::std::os::raw::c_ushort,
        nMonth: ::std::os::raw::c_ushort,
        nYear: ::std::os::raw::c_ushort,
    ) -> __BindgenBitfieldUnit<[u8; 3usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 3usize]> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 3u8, {
            let nWeekDay: u16 = unsafe { ::std::mem::transmute(nWeekDay) };
            nWeekDay as u64
        });
        __bindgen_bitfield_unit.set(3usize, 6u8, {
            let nMonthDay: u16 = unsafe { ::std::mem::transmute(nMonthDay) };
            nMonthDay as u64
        });
        __bindgen_bitfield_unit.set(9usize, 5u8, {
            let nMonth: u16 = unsafe { ::std::mem::transmute(nMonth) };
            nMonth as u64
        });
        __bindgen_bitfield_unit.set(16usize, 8u8, {
            let nYear: u16 = unsafe { ::std::mem::transmute(nYear) };
            nYear as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[repr(align(2))]
#[derive(Debug, Default, Copy, Clone)]
pub struct Date2 {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[test]
fn bindgen_test_layout_Date2() {
    assert_eq!(
        ::std::mem::size_of::<Date2>(),
        4usize,
        concat!("Size of: ", stringify!(Date2))
    );
    assert_eq!(
        ::std::mem::align_of::<Date2>(),
        2usize,
        concat!("Alignment of ", stringify!(Date2))
    );
}
impl Date2 {
    #[inline]
    pub fn nWeekDay(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 3u8) as u16)
        }
    }
    #[inline]
    pub fn set_nWeekDay(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn nMonthDay(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(3usize, 6u8) as u16)
        }
    }
    #[inline]
    pub fn set_nMonthDay(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub fn nMonth(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(9usize, 5u8) as u16)
        }
    }
    #[inline]
    pub fn set_nMonth(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(9usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub fn nYear(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(16usize, 8u8) as u16)
        }
    }
    #[inline]
    pub fn set_nYear(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn byte(&self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(24usize, 8u8) as u8)
        }
    }
    #[inline]
    pub fn set_byte(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(24usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        nWeekDay: ::std::os::raw::c_ushort,
        nMonthDay: ::std::os::raw::c_ushort,
        nMonth: ::std::os::raw::c_ushort,
        nYear: ::std::os::raw::c_ushort,
        byte: ::std::os::raw::c_uchar,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 3u8, {
            let nWeekDay: u16 = unsafe { ::std::mem::transmute(nWeekDay) };
            nWeekDay as u64
        });
        __bindgen_bitfield_unit.set(3usize, 6u8, {
            let nMonthDay: u16 = unsafe { ::std::mem::transmute(nMonthDay) };
            nMonthDay as u64
        });
        __bindgen_bitfield_unit.set(9usize, 5u8, {
            let nMonth: u16 = unsafe { ::std::mem::transmute(nMonth) };
            nMonth as u64
        });
        __bindgen_bitfield_unit.set(16usize, 8u8, {
            let nYear: u16 = unsafe { ::std::mem::transmute(nYear) };
            nYear as u64
        });
        __bindgen_bitfield_unit.set(24usize, 8u8, {
            let byte: u8 = unsafe { ::std::mem::transmute(byte) };
            byte as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[repr(align(2))]
#[derive(Debug, Default, Copy, Clone)]
pub struct Date3 {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 3usize]>,
    pub byte: ::std::os::raw::c_uchar,
}
#[test]
fn bindgen_test_layout_Date3() {
    assert_eq!(
        ::std::mem::size_of::<Date3>(),
        4usize,
        concat!("Size of: ", stringify!(Date3))
    );
    assert_eq!(
        ::std::mem::align_of::<Date3>(),
        2usize,
        concat!("Alignment of ", stringify!(Date3))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Date3>())).byte as *const _ as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(Date3),
            "::",
            stringify!(byte)
        )
    );
}
impl Date3 {
    #[inline]
    pub fn nWeekDay(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 3u8) as u16)
        }
    }
    #[inline]
    pub fn set_nWeekDay(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn nMonthDay(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(3usize, 6u8) as u16)
        }
    }
    #[inline]
    pub fn set_nMonthDay(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub fn nMonth(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(9usize, 5u8) as u16)
        }
    }
    #[inline]
    pub fn set_nMonth(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(9usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub fn nYear(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(16usize, 8u8) as u16)
        }
    }
    #[inline]
    pub fn set_nYear(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        nWeekDay: ::std::os::raw::c_ushort,
        nMonthDay: ::std::os::raw::c_ushort,
        nMonth: ::std::os::raw::c_ushort,
        nYear: ::std::os::raw::c_ushort,
    ) -> __BindgenBitfieldUnit<[u8; 3usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 3usize]> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 3u8, {
            let nWeekDay: u16 = unsafe { ::std::mem::transmute(nWeekDay) };
            nWeekDay as u64
        });
        __bindgen_bitfield_unit.set(3usize, 6u8, {
            let nMonthDay: u16 = unsafe { ::std::mem::transmute(nMonthDay) };
            nMonthDay as u64
        });
        __bindgen_bitfield_unit.set(9usize, 5u8, {
            let nMonth: u16 = unsafe { ::std::mem::transmute(nMonth) };
            nMonth as u64
        });
        __bindgen_bitfield_unit.set(16usize, 8u8, {
            let nYear: u16 = unsafe { ::std::mem::transmute(nYear) };
            nYear as u64
        });
        __bindgen_bitfield_unit
    }
}
