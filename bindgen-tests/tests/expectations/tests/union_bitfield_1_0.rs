#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub fn new(storage: Storage) -> Self {
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
#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct U4 {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenUnionField<__BindgenBitfieldUnit<[u8; 1usize]>>,
    pub bindgen_union_field: u32,
}
#[test]
fn bindgen_test_layout_U4() {
    assert_eq!(::std::mem::size_of::<U4>(), 4usize, "Size of U4");
    assert_eq!(::std::mem::align_of::<U4>(), 4usize, "Alignment of U4");
}
impl Clone for U4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl U4 {
    #[inline]
    pub fn derp(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.as_ref().get(0usize, 1u8) as u32)
        }
    }
    #[inline]
    pub fn set_derp(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.as_mut().set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn derp_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 1usize],
                >>::raw_get(
                    (*::std::ptr::addr_of!((*this)._bitfield_1)).as_ref() as *const _,
                    0usize,
                    1u8,
                ) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_derp_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 1usize],
            >>::raw_set(
                (*::std::ptr::addr_of_mut!((*this)._bitfield_1)).as_mut() as *mut _,
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        derp: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                1u8,
                {
                    let derp: u32 = unsafe { ::std::mem::transmute(derp) };
                    derp as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct B {
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenUnionField<__BindgenBitfieldUnit<[u8; 4usize]>>,
    pub bindgen_union_field: u32,
}
#[test]
fn bindgen_test_layout_B() {
    assert_eq!(::std::mem::size_of::<B>(), 4usize, "Size of B");
    assert_eq!(::std::mem::align_of::<B>(), 4usize, "Alignment of B");
}
impl Clone for B {
    fn clone(&self) -> Self {
        *self
    }
}
impl B {
    #[inline]
    pub fn foo(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.as_ref().get(0usize, 31u8) as u32)
        }
    }
    #[inline]
    pub fn set_foo(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.as_mut().set(0usize, 31u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn foo_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get(
                    (*::std::ptr::addr_of!((*this)._bitfield_1)).as_ref() as *const _,
                    0usize,
                    31u8,
                ) as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_foo_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set(
                (*::std::ptr::addr_of_mut!((*this)._bitfield_1)).as_mut() as *mut _,
                0usize,
                31u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn bar(&self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.as_ref().get(31usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_bar(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.as_mut().set(31usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn bar_raw(this: *const Self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get(
                    (*::std::ptr::addr_of!((*this)._bitfield_1)).as_ref() as *const _,
                    31usize,
                    1u8,
                ) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_bar_raw(this: *mut Self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set(
                (*::std::ptr::addr_of_mut!((*this)._bitfield_1)).as_mut() as *mut _,
                31usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        foo: ::std::os::raw::c_uint,
        bar: ::std::os::raw::c_uchar,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                31u8,
                {
                    let foo: u32 = unsafe { ::std::mem::transmute(foo) };
                    foo as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                31usize,
                1u8,
                {
                    let bar: u8 = unsafe { ::std::mem::transmute(bar) };
                    bar as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct HasBigBitfield {
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenUnionField<__BindgenBitfieldUnit<[u8; 16usize]>>,
    pub bindgen_union_field: [u8; 16usize],
}
#[test]
fn bindgen_test_layout_HasBigBitfield() {
    assert_eq!(
        ::std::mem::size_of::<HasBigBitfield>(),
        16usize,
        "Size of HasBigBitfield",
    );
}
impl Clone for HasBigBitfield {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for HasBigBitfield {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
impl ::std::cmp::PartialEq for HasBigBitfield {
    fn eq(&self, other: &HasBigBitfield) -> bool {
        &self.bindgen_union_field[..] == &other.bindgen_union_field[..]
    }
}
