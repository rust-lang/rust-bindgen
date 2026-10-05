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
#[derive(Debug, Default, Copy, Clone)]
pub struct pad_me {
    pub first: u8,
    pub __bindgen_padding_0: [u8; 3usize],
    pub second: u32,
    pub third: u16,
    pub __bindgen_padding_1: [u8; 2usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of pad_me"][::std::mem::size_of::<pad_me>() - 12usize];
    ["Alignment of pad_me"][::std::mem::align_of::<pad_me>() - 4usize];
    ["Offset of field: pad_me::first"][::std::mem::offset_of!(pad_me, first) - 0usize];
    ["Offset of field: pad_me::second"][::std::mem::offset_of!(pad_me, second) - 4usize];
    ["Offset of field: pad_me::third"][::std::mem::offset_of!(pad_me, third) - 8usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub union dont_pad_me {
    pub first: u8,
    pub second: u32,
    pub third: u16,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of dont_pad_me"][::std::mem::size_of::<dont_pad_me>() - 4usize];
    ["Alignment of dont_pad_me"][::std::mem::align_of::<dont_pad_me>() - 4usize];
    [
        "Offset of field: dont_pad_me::first",
    ][::std::mem::offset_of!(dont_pad_me, first) - 0usize];
    [
        "Offset of field: dont_pad_me::second",
    ][::std::mem::offset_of!(dont_pad_me, second) - 0usize];
    [
        "Offset of field: dont_pad_me::third",
    ][::std::mem::offset_of!(dont_pad_me, third) - 0usize];
};
impl Default for dont_pad_me {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct also_pad_me {
    pub first: u16,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub __bindgen_padding_0: u8,
}
#[test]
fn bindgen_test_layout_also_pad_me() {
    const UNINIT: ::std::mem::MaybeUninit<also_pad_me> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<also_pad_me>(),
        4usize,
        concat!("Size of: ", stringify!(also_pad_me))
    );
    assert_eq!(
        ::std::mem::align_of::<also_pad_me>(),
        2usize,
        concat!("Alignment of ", stringify!(also_pad_me))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).first) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(also_pad_me),
            "::",
            stringify!(first)
        )
    );
}
impl also_pad_me {
    #[inline]
    pub fn bits(&self) -> u8 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_bits(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(bits: u8) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let bits: u8 = unsafe { ::std::mem::transmute(bits) };
            bits as u64
        });
        __bindgen_bitfield_unit
    }
}
