#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq)]
pub struct Doggo {
    pub x: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<Doggo>() == 4usize, "Size of Doggo");
    assert!(::std::mem::align_of::<Doggo>() == 4usize, "Alignment of Doggo");
    assert!(::std::mem::offset_of!(Doggo, x) == 0usize, "Offset of field: Doggo::x");
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq)]
pub struct Null {
    pub _address: u8,
}
const _: () = {
    assert!(::std::mem::size_of::<Null>() == 1usize, "Size of Null");
    assert!(::std::mem::align_of::<Null>() == 1usize, "Alignment of Null");
};
/** This type is an opaque union. Unions can't derive anything interesting like
 Debug or Default, even if their layout can, because it would require knowing
 which variant is in use. Opaque unions still end up as a `union` in the Rust
 bindings, but they just have one variant. Even so, can't derive. We should
 probably emit an opaque struct for opaque unions... but until then, we have
 this test to make sure that opaque unions don't derive and still compile.*/
#[repr(C)]
#[repr(align(4))]
#[derive(Copy, Clone)]
pub union DoggoOrNull {
    pub _bindgen_opaque_blob: u32,
}
const _: () = {
    assert!(::std::mem::size_of::<DoggoOrNull>() == 4usize, "Size of DoggoOrNull");
    assert!(::std::mem::align_of::<DoggoOrNull>() == 4usize, "Alignment of DoggoOrNull");
};
impl Default for DoggoOrNull {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
