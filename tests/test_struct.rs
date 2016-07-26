use std::mem;

use support::assert_bind_eq;

#[test]
fn with_anon_enum() {
    assert_bind_eq(Default::default(), "headers/struct_with_anon_enum.h", "
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct foo {
            pub bar: Enum_Unnamed1,
        }
        impl ::std::default::Default for foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
        #[derive(Copy, Clone)]
        #[repr(u32)]
        #[derive(Debug)]
        pub enum Enum_Unnamed1 {
            FOO_OPTION_1 = 0,
            FOO_OPTION_2 = 1,
            FOO_OPTION_3 = 2,
        }
    ");
}

#[test]
fn with_anon_enum_bitfields() {
    assert_bind_eq(Default::default(), "headers/struct_with_anon_enum_bitfields.h", "
        #[derive(Copy, Clone)]
        #[repr(u32)]
        #[derive(Debug)]
        pub enum test {
            TEST_OPTION_1 = 0,
            TEST_OPTION_2 = 1,
            TEST_OPTION_3 = 2,
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct foo {
            pub _bindgen_bitfield_1_: Enum_Unnamed1,
            pub _bindgen_bitfield_2_: test,
        }
        impl ::std::default::Default for foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
        #[derive(Copy, Clone)]
        #[repr(u32)]
        #[derive(Debug)]
        pub enum Enum_Unnamed1 {
            FOO_OPTION_1 = 0,
            FOO_OPTION_2 = 1,
            FOO_OPTION_3 = 2,
        }
    ");
}

#[test]
fn with_anon_struct() {
    assert_bind_eq(Default::default(), "headers/struct_with_anon_struct.h", "
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct foo {
            pub bar: Struct_Unnamed1,
        }
        impl ::std::default::Default for foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct Struct_Unnamed1 {
            pub a: ::std::os::raw::c_int,
            pub b: ::std::os::raw::c_int,
        }
        impl ::std::default::Default for Struct_Unnamed1 {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn with_anon_struct_array() {
    assert_bind_eq(Default::default(), "headers/struct_with_anon_struct_array.h", "
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct foo {
            pub bar: [Struct_Unnamed1; 2usize],
            pub baz: [[[Struct_Unnamed2; 4usize]; 3usize]; 2usize],
        }
        impl ::std::default::Default for foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }

        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct Struct_Unnamed1 {
            pub a: ::std::os::raw::c_int,
            pub b: ::std::os::raw::c_int,
        }
        impl ::std::default::Default for Struct_Unnamed1 {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }

        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct Struct_Unnamed2 {
            pub a: ::std::os::raw::c_int,
            pub b: ::std::os::raw::c_int,
        }
        impl ::std::default::Default for Struct_Unnamed2 {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn with_anon_struct_pointer() {
    assert_bind_eq(Default::default(), "headers/struct_with_anon_struct_pointer.h", "
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct foo {
            pub bar: *mut Struct_Unnamed1,
        }
        impl ::std::default::Default for foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct Struct_Unnamed1 {
            pub a: ::std::os::raw::c_int,
            pub b: ::std::os::raw::c_int,
        }
        impl ::std::default::Default for Struct_Unnamed1 {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn with_anon_union() {
    assert_bind_eq(Default::default(), "headers/struct_with_anon_union.h", "
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct foo {
            pub bar: Union_Unnamed1,
        }
        impl ::std::default::Default for foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct Union_Unnamed1 {
            pub _bindgen_data_: [u32; 1usize],
        }
        impl Union_Unnamed1 {
            pub unsafe fn a(&mut self) -> *mut ::std::os::raw::c_uint {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(0))
            }
            pub unsafe fn b(&mut self) -> *mut ::std::os::raw::c_ushort {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(0))
            }
        }
        impl ::std::default::Default for Union_Unnamed1 {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn with_anon_unnamed_struct() {
    assert_bind_eq(Default::default(), "headers/struct_with_anon_unnamed_struct.h", "
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct foo {
            pub _bindgen_data_1_: [u32; 2usize],
        }
        impl foo {
            pub unsafe fn a(&mut self) -> *mut ::std::os::raw::c_uint {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
                ::std::mem::transmute(raw.offset(0))
            }
            pub unsafe fn b(&mut self) -> *mut ::std::os::raw::c_uint {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
                ::std::mem::transmute(raw.offset(4))
            }
        }
        impl ::std::default::Default for foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn with_anon_unnamed_union() {
    assert_bind_eq(Default::default(), "headers/struct_with_anon_unnamed_union.h", "
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct foo {
            pub _bindgen_data_1_: [u32; 1usize],
        }
        impl foo {
            pub unsafe fn a(&mut self) -> *mut ::std::os::raw::c_uint {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
                ::std::mem::transmute(raw.offset(0))
            }
            pub unsafe fn b(&mut self) -> *mut ::std::os::raw::c_ushort {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
                ::std::mem::transmute(raw.offset(0))
            }
        }
        impl ::std::default::Default for foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn with_nesting() {
    assert_bind_eq(Default::default(), "headers/struct_with_nesting.h", "
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct foo {
            pub a: ::std::os::raw::c_uint,
            pub _bindgen_data_1_: [u32; 1usize],
        }
        impl foo {
            pub unsafe fn b(&mut self) -> *mut ::std::os::raw::c_uint {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
                ::std::mem::transmute(raw.offset(0))
            }
            pub unsafe fn c1(&mut self) -> *mut ::std::os::raw::c_ushort {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
                ::std::mem::transmute(raw.offset(0))
            }
            pub unsafe fn c2(&mut self) -> *mut ::std::os::raw::c_ushort {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
                ::std::mem::transmute(raw.offset(2))
            }
            pub unsafe fn d1(&mut self) -> *mut ::std::os::raw::c_uchar {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
                ::std::mem::transmute(raw.offset(0))
            }
            pub unsafe fn d2(&mut self) -> *mut ::std::os::raw::c_uchar {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
                ::std::mem::transmute(raw.offset(1))
            }
            pub unsafe fn d3(&mut self) -> *mut ::std::os::raw::c_uchar {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
                ::std::mem::transmute(raw.offset(2))
            }
            pub unsafe fn d4(&mut self) -> *mut ::std::os::raw::c_uchar {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
                ::std::mem::transmute(raw.offset(3))
            }
        }
        impl ::std::default::Default for foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn containing_fwd_decl_struct() {
    assert_bind_eq(Default::default(), "headers/struct_containing_forward_declared_struct.h", "
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct a {
            pub val_a: *mut b,
        }
        impl ::std::default::Default for a {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }

        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct b {
            pub val_b: ::std::os::raw::c_int,
        }
        impl ::std::default::Default for b {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn with_bitfields() {
    assert_bind_eq(Default::default(), "headers/struct_with_bitfields.h", "
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct bitfield {
            pub _bindgen_bitfield_1_: ::std::os::raw::c_ushort,
            pub e: ::std::os::raw::c_int,
            pub _bindgen_bitfield_2_: ::std::os::raw::c_uint,
            pub _bindgen_bitfield_3_: ::std::os::raw::c_uint,
        }
        impl ::std::default::Default for bitfield {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn with_fwd_decl_struct() {
    assert_bind_eq(Default::default(), "headers/forward_declared_struct.h", "
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct a {
            pub b: ::std::os::raw::c_int,
        }
        impl ::std::default::Default for a {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct c {
            pub d: ::std::os::raw::c_int,
        }
        impl ::std::default::Default for c {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}


#[test]
fn packed_struct() {
    assert_bind_eq(Default::default(), "headers/struct_with_packing.h", "
        #[repr(C, packed)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct a {
            pub b: ::std::os::raw::c_char,
            pub c: ::std::os::raw::c_short,
        }
        impl ::std::default::Default for a {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn derive_debug_big_array() {
    assert_bind_eq(Default::default(), "headers/struct_with_derive_debug.h", "
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct LittleArray {
            pub a: [::std::os::raw::c_int; 32usize],
        }
        impl ::std::default::Default for LittleArray {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
        #[repr(C)]
        #[derive(Copy)]
        pub struct BigArray {
            pub a: [::std::os::raw::c_int; 33usize],
        }
        impl ::std::clone::Clone for BigArray {
            fn clone(&self) -> Self { *self  }
        }
        impl ::std::default::Default for BigArray {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct WithLittleArray {
            pub a: LittleArray,
        }
        impl ::std::default::Default for WithLittleArray {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
        #[repr(C)]
        #[derive(Copy)]
        pub struct WithBigArray {
            pub a: BigArray,
        }
        impl ::std::clone::Clone for WithBigArray {
            fn clone(&self) -> Self { *self  }
        }
        impl ::std::default::Default for WithBigArray {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn struct_with_incomplete_array() {
    assert_bind_eq(Default::default(), "headers/struct_with_incomplete_array.h", "
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct incomplete_array {
            pub x: ::std::os::raw::c_int,
            pub y: [::std::os::raw::c_int; 0usize],
        }
        impl ::std::default::Default for incomplete_array {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

macro_rules! offset_of_unsafe {
    ($container:path, $field:ident) => {{
        let $container { $field : _, .. };

        &(*(0 as *const $container)).$field as *const _ as isize
    }};
}

macro_rules! offset_of {
    ($container:path, $field:ident) => {
        unsafe { offset_of_unsafe!($container, $field) }
    };
}

#[test]
fn struct_with_aligned_struct() {
    #![allow(non_camel_case_types)]

    assert_bind_eq(Default::default(), "headers/struct_with_aligned_struct.h", "
        pub type int16_t = i16;
        pub type int32_t = i32;
        pub type int64_t = i64;
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct foo {
            pub x: int32_t,
            pub y: int64_t,
            pub z: int16_t,
            _bindgen_padding_0_: [u64; 5usize],
        }
        impl ::std::default::Default for foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct bar {
            pub a: int32_t,
            pub b: int64_t,
            _bindgen_padding_0_: [u64; 6usize],
            pub foo: foo,
        }
        impl ::std::default::Default for bar {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
		#[repr(C)]
		#[derive(Copy, Clone)]
		#[derive(Debug)]
		pub struct smaller_align {
			pub x: int32_t,
			pub y: int64_t,
			pub z: int16_t,
			_bindgen_padding_0_: [u8; 14usize],
		}
		impl ::std::default::Default for smaller_align {
			fn default() -> Self { unsafe { ::std::mem::zeroed() } }
		}
		#[repr(C)]
		#[derive(Copy, Clone)]
		#[derive(Debug)]
		pub struct implicit_pad {
			pub a: int32_t,
			pub b: int32_t,
			pub c: int16_t,
		}
		impl ::std::default::Default for implicit_pad {
			fn default() -> Self { unsafe { ::std::mem::zeroed() } }
		}

    ");

    pub type int16_t = ::std::os::raw::c_short;
    pub type int32_t = ::std::os::raw::c_int;
    pub type int64_t = ::std::os::raw::c_longlong;
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[derive(Debug)]
    pub struct foo {
        pub x: int32_t,
        pub y: int64_t,
        pub z: int16_t,
        _bindgen_padding_0_: [u64; 5usize],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    #[derive(Debug)]
    pub struct bar {
        pub a: int32_t,
        pub b: int64_t,
        _bindgen_padding_0_: [u64; 6usize],
        pub foo: foo,
    }

    assert_eq!(mem::size_of::<foo>(), 64);
    assert_eq!(offset_of!(foo, x), 0);
    assert_eq!(offset_of!(foo, y), 8);
    assert_eq!(offset_of!(foo, z), 16);
    assert_eq!(offset_of!(foo, _bindgen_padding_0_), 24);

    assert_eq!(mem::size_of::<bar>(), 128);
    assert_eq!(offset_of!(bar, a), 0);
    assert_eq!(offset_of!(bar, b), 8);
    assert_eq!(offset_of!(bar, _bindgen_padding_0_), 16);
    assert_eq!(offset_of!(bar, foo), 64);
}
