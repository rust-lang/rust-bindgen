use support::assert_bind_eq;

#[test]
fn with_anon_struct() {
    assert_bind_eq(Default::default(), "headers/union_with_anon_struct.h", "
        #[repr(C)]
        #[derive(Copy)]
        pub struct Union_foo {
            pub _bindgen_data_: [u32; 2usize],
        }
        impl Union_foo {
            pub unsafe fn bar(&mut self) -> *mut Struct_Unnamed1 {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(0))
            }
        }
        impl ::std::clone::Clone for Union_foo {
            fn clone(&self) -> Self { *self }
        }
        impl ::std::default::Default for Union_foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
        #[repr(C)]
        #[derive(Copy)]
        #[derive(Debug)]
        pub struct Struct_Unnamed1 {
            pub a: ::std::os::raw::c_uint,
            pub b: ::std::os::raw::c_uint,
        }
        impl ::std::clone::Clone for Struct_Unnamed1 {
            fn clone(&self) -> Self { *self }
        }
        impl ::std::default::Default for Struct_Unnamed1 {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn with_anon_struct_bitfield() {
    assert_bind_eq(Default::default(), "headers/union_with_anon_struct_bitfield.h", "
        #[repr(C)]
        #[derive(Copy)]
        pub struct Union_foo {
            pub _bindgen_data_: [u32; 1usize],
        }

        impl Union_foo {
            pub unsafe fn a(&mut self) -> *mut ::std::os::raw::c_int {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(0))
            }
        }

        impl ::std::clone::Clone for Union_foo {
            fn clone(&self) -> Self { *self }
        }

        impl ::std::default::Default for Union_foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn with_anon_union() {
    assert_bind_eq(Default::default(), "headers/union_with_anon_union.h", "
        #[repr(C)]
        #[derive(Copy)]
        pub struct Union_foo {
            pub _bindgen_data_: [u32; 1usize],
        }
        impl Union_foo {
            pub unsafe fn bar(&mut self) -> *mut Union_Unnamed1 {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(0))
            }
        }
        impl ::std::clone::Clone for Union_foo {
            fn clone(&self) -> Self { *self }
        }
        impl ::std::default::Default for Union_foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
        #[repr(C)]
        #[derive(Copy)]
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
        impl ::std::clone::Clone for Union_Unnamed1 {
            fn clone(&self) -> Self { *self }
        }
        impl ::std::default::Default for Union_Unnamed1 {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn with_anon_unnamed_struct() {
    assert_bind_eq(Default::default(), "headers/union_with_anon_unnamed_struct.h", "
        #[repr(C)]
        #[derive(Copy)]
        pub struct Union_pixel {
            pub _bindgen_data_: [u32; 1usize],
        }
        impl Union_pixel {
            pub unsafe fn rgba(&mut self) -> *mut ::std::os::raw::c_uint {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(0))
            }
            pub unsafe fn r(&mut self) -> *mut ::std::os::raw::c_uchar {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(0))
            }
            pub unsafe fn g(&mut self) -> *mut ::std::os::raw::c_uchar {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(1))
            }
            pub unsafe fn b(&mut self) -> *mut ::std::os::raw::c_uchar {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(2))
            }
            pub unsafe fn a(&mut self) -> *mut ::std::os::raw::c_uchar {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(3))
            }
        }
        impl ::std::clone::Clone for Union_pixel {
            fn clone(&self) -> Self { *self }
        }
        impl ::std::default::Default for Union_pixel {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn with_anon_unnamed_union() {
    assert_bind_eq(Default::default(), "headers/union_with_anon_unnamed_union.h", "
        #[repr(C)]
        #[derive(Copy)]
        pub struct Union_foo {
            pub _bindgen_data_: [u32; 1usize],
        }
        impl Union_foo {
            pub unsafe fn a(&mut self) -> *mut ::std::os::raw::c_uint {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(0))
            }
            pub unsafe fn b(&mut self) -> *mut ::std::os::raw::c_ushort {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(0))
            }
            pub unsafe fn c(&mut self) -> *mut ::std::os::raw::c_uchar {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(0))
            }
        }
        impl ::std::clone::Clone for Union_foo {
            fn clone(&self) -> Self { *self }
        }
        impl ::std::default::Default for Union_foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn with_nesting() {
    assert_bind_eq(Default::default(), "headers/union_with_nesting.h", "
        #[repr(C)]
        #[derive(Copy)]
        pub struct Union_foo {
            pub _bindgen_data_: [u32; 1usize],
        }
        impl Union_foo {
            pub unsafe fn a(&mut self) -> *mut ::std::os::raw::c_uint {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(0))
            }
            pub unsafe fn b1(&mut self) -> *mut ::std::os::raw::c_ushort {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(0))
            }
            pub unsafe fn b2(&mut self) -> *mut ::std::os::raw::c_ushort {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(0))
            }
            pub unsafe fn c1(&mut self) -> *mut ::std::os::raw::c_ushort {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(2))
            }
            pub unsafe fn c2(&mut self) -> *mut ::std::os::raw::c_ushort {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(2))
            }
        }
        impl ::std::clone::Clone for Union_foo {
            fn clone(&self) -> Self { *self }
        }
        impl ::std::default::Default for Union_foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}
