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
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PubPriv {
    pub x: ::std::os::raw::c_int,
    y: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of PubPriv"][::std::mem::size_of::<PubPriv>() - 8usize];
    ["Alignment of PubPriv"][::std::mem::align_of::<PubPriv>() - 4usize];
    ["Offset of field: PubPriv::x"][::std::mem::offset_of!(PubPriv, x) - 0usize];
    ["Offset of field: PubPriv::y"][::std::mem::offset_of!(PubPriv, y) - 4usize];
};
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct PrivateBitFields {
    pub _bitfield_align_1: [u8; 0],
    _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub __bindgen_padding_0: [u8; 3usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of PrivateBitFields"][::std::mem::size_of::<PrivateBitFields>() - 4usize];
    [
        "Alignment of PrivateBitFields",
    ][::std::mem::align_of::<PrivateBitFields>() - 4usize];
};
impl PrivateBitFields {
    #[inline]
    fn a(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u32) }
    }
    #[inline]
    fn set_a(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    fn b(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u32) }
    }
    #[inline]
    fn set_b(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    fn new_bitfield_1(
        a: ::std::os::raw::c_uint,
        b: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                4u8,
                {
                    let a: u32 = unsafe { ::std::mem::transmute(a) };
                    a as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                4usize,
                4u8,
                {
                    let b: u32 = unsafe { ::std::mem::transmute(b) };
                    b as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct PublicBitFields {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub __bindgen_padding_0: [u8; 3usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of PublicBitFields"][::std::mem::size_of::<PublicBitFields>() - 4usize];
    ["Alignment of PublicBitFields"][::std::mem::align_of::<PublicBitFields>() - 4usize];
};
impl PublicBitFields {
    #[inline]
    pub fn a(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u32) }
    }
    #[inline]
    pub fn set_a(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn b(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u32) }
    }
    #[inline]
    pub fn set_b(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        a: ::std::os::raw::c_uint,
        b: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                4u8,
                {
                    let a: u32 = unsafe { ::std::mem::transmute(a) };
                    a as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                4usize,
                4u8,
                {
                    let b: u32 = unsafe { ::std::mem::transmute(b) };
                    b as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct MixedBitFields {
    pub _bitfield_align_1: [u8; 0],
    _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub __bindgen_padding_0: [u8; 3usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MixedBitFields"][::std::mem::size_of::<MixedBitFields>() - 4usize];
    ["Alignment of MixedBitFields"][::std::mem::align_of::<MixedBitFields>() - 4usize];
};
impl MixedBitFields {
    #[inline]
    fn a(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u32) }
    }
    #[inline]
    fn set_a(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn d(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u32) }
    }
    #[inline]
    pub fn set_d(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    fn new_bitfield_1(
        a: ::std::os::raw::c_uint,
        d: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                4u8,
                {
                    let a: u32 = unsafe { ::std::mem::transmute(a) };
                    a as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                4usize,
                4u8,
                {
                    let d: u32 = unsafe { ::std::mem::transmute(d) };
                    d as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Base {
    pub member: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Base"][::std::mem::size_of::<Base>() - 4usize];
    ["Alignment of Base"][::std::mem::align_of::<Base>() - 4usize];
    ["Offset of field: Base::member"][::std::mem::offset_of!(Base, member) - 0usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct InheritsPrivately {
    _base: Base,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of InheritsPrivately"][::std::mem::size_of::<InheritsPrivately>() - 4usize];
    [
        "Alignment of InheritsPrivately",
    ][::std::mem::align_of::<InheritsPrivately>() - 4usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct InheritsPublically {
    pub _base: Base,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of InheritsPublically"][::std::mem::size_of::<InheritsPublically>() - 4usize];
    [
        "Alignment of InheritsPublically",
    ][::std::mem::align_of::<InheritsPublically>() - 4usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct WithAnonStruct {
    __bindgen_anon_1: WithAnonStruct__bindgen_ty_1,
    pub __bindgen_anon_2: WithAnonStruct__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct WithAnonStruct__bindgen_ty_1 {
    pub a: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of WithAnonStruct__bindgen_ty_1",
    ][::std::mem::size_of::<WithAnonStruct__bindgen_ty_1>() - 4usize];
    [
        "Alignment of WithAnonStruct__bindgen_ty_1",
    ][::std::mem::align_of::<WithAnonStruct__bindgen_ty_1>() - 4usize];
    [
        "Offset of field: WithAnonStruct__bindgen_ty_1::a",
    ][::std::mem::offset_of!(WithAnonStruct__bindgen_ty_1, a) - 0usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct WithAnonStruct__bindgen_ty_2 {
    pub b: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of WithAnonStruct__bindgen_ty_2",
    ][::std::mem::size_of::<WithAnonStruct__bindgen_ty_2>() - 4usize];
    [
        "Alignment of WithAnonStruct__bindgen_ty_2",
    ][::std::mem::align_of::<WithAnonStruct__bindgen_ty_2>() - 4usize];
    [
        "Offset of field: WithAnonStruct__bindgen_ty_2::b",
    ][::std::mem::offset_of!(WithAnonStruct__bindgen_ty_2, b) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of WithAnonStruct"][::std::mem::size_of::<WithAnonStruct>() - 8usize];
    ["Alignment of WithAnonStruct"][::std::mem::align_of::<WithAnonStruct>() - 4usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WithAnonUnion {
    __bindgen_anon_1: WithAnonUnion__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union WithAnonUnion__bindgen_ty_1 {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of WithAnonUnion__bindgen_ty_1",
    ][::std::mem::size_of::<WithAnonUnion__bindgen_ty_1>() - 1usize];
    [
        "Alignment of WithAnonUnion__bindgen_ty_1",
    ][::std::mem::align_of::<WithAnonUnion__bindgen_ty_1>() - 1usize];
};
impl Default for WithAnonUnion__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of WithAnonUnion"][::std::mem::size_of::<WithAnonUnion>() - 1usize];
    ["Alignment of WithAnonUnion"][::std::mem::align_of::<WithAnonUnion>() - 1usize];
};
impl Default for WithAnonUnion {
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
pub struct Override {
    pub a: ::std::os::raw::c_uint,
    /// <div rustbindgen private></div>
    b: ::std::os::raw::c_uint,
    private_c: ::std::os::raw::c_uint,
    pub _bitfield_align_1: [u8; 0],
    _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize]>,
    pub __bindgen_padding_0: u16,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Override"][::std::mem::size_of::<Override>() - 16usize];
    ["Alignment of Override"][::std::mem::align_of::<Override>() - 4usize];
    ["Offset of field: Override::a"][::std::mem::offset_of!(Override, a) - 0usize];
    ["Offset of field: Override::b"][::std::mem::offset_of!(Override, b) - 4usize];
    [
        "Offset of field: Override::private_c",
    ][::std::mem::offset_of!(Override, private_c) - 8usize];
};
impl Override {
    #[inline]
    pub fn bf_a(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u32) }
    }
    #[inline]
    pub fn set_bf_a(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    fn bf_b(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u32) }
    }
    #[inline]
    fn set_bf_b(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    fn private_bf_c(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 4u8) as u32) }
    }
    #[inline]
    fn set_private_bf_c(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 4u8, val as u64)
        }
    }
    #[inline]
    fn new_bitfield_1(
        bf_a: ::std::os::raw::c_uint,
        bf_b: ::std::os::raw::c_uint,
        private_bf_c: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 2usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                4u8,
                {
                    let bf_a: u32 = unsafe { ::std::mem::transmute(bf_a) };
                    bf_a as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                4usize,
                4u8,
                {
                    let bf_b: u32 = unsafe { ::std::mem::transmute(bf_b) };
                    bf_b as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                8usize,
                4u8,
                {
                    let private_bf_c: u32 = unsafe {
                        ::std::mem::transmute(private_bf_c)
                    };
                    private_bf_c as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
