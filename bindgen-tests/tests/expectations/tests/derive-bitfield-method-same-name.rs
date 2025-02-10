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
}
/// Because this struct have array larger than 32 items
/// and --with-derive-partialeq --impl-partialeq --impl-debug is provided,
/// this struct should manually implement `Debug` and `PartialEq`.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Foo {
    pub large: [::std::os::raw::c_int; 33usize],
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize]>,
    pub __bindgen_padding_0: u16,
}
#[test]
fn bindgen_test_layout_Foo() {
    const UNINIT: ::std::mem::MaybeUninit<Foo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<Foo>(), 136usize, "Size of Foo");
    assert_eq!(::std::mem::align_of::<Foo>(), 4usize, "Alignment of Foo");
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).large) as usize - ptr as usize },
        0usize,
        "Offset of field: Foo::large",
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN3Foo4typeEv"]
    pub fn Foo_type(this: *mut Foo) -> ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_ZN3Foo9set_type_Ec"]
    pub fn Foo_set_type_(this: *mut Foo, c: ::std::os::raw::c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZN3Foo8set_typeEc"]
    pub fn Foo_set_type(this: *mut Foo, c: ::std::os::raw::c_char);
}
impl Default for Foo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for Foo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "Foo {{ large: [{}], type_ : {:?},  }}",
            {
                use std::fmt::Write as _;
                let mut output = String::new();
                let mut iter = self.large.iter();
                if let Some(value) = iter.next() {
                    let _ = write!(output, "{value:?}");
                    for value in iter {
                        let _ = write!(output, ", {value:?}");
                    }
                }
                output
            },
            self.type__bindgen_bitfield(),
        )
    }
}
impl ::std::cmp::PartialEq for Foo {
    fn eq(&self, other: &Foo) -> bool {
        &self.large[..] == &other.large[..]
            && self.type__bindgen_bitfield() == other.type__bindgen_bitfield()
    }
}
impl Foo {
    #[inline]
    pub fn type__bindgen_bitfield(&self) -> ::std::os::raw::c_char {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 3u8) as u8) }
    }
    #[inline]
    pub fn set_type__bindgen_bitfield(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        type__bindgen_bitfield: ::std::os::raw::c_char,
    ) -> __BindgenBitfieldUnit<[u8; 2usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                3u8,
                {
                    let type__bindgen_bitfield: u8 = unsafe {
                        ::std::mem::transmute(type__bindgen_bitfield)
                    };
                    type__bindgen_bitfield as u64
                },
            );
        __bindgen_bitfield_unit
    }
    #[inline]
    pub unsafe fn type_(&mut self) -> ::std::os::raw::c_char {
        Foo_type(self)
    }
    #[inline]
    pub unsafe fn set_type_(&mut self, c: ::std::os::raw::c_char) {
        Foo_set_type_(self, c)
    }
    #[inline]
    pub unsafe fn set_type(&mut self, c: ::std::os::raw::c_char) {
        Foo_set_type(self, c)
    }
}
