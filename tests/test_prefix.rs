use bindgen::{Builder,BindgenOptions};
use support::assert_bind_eq;

#[test]
fn remove_prefix() {
    let opts = BindgenOptions {
        remove_prefix: "test_".into(),
        ..Default::default()
    };
    assert_bind_eq(opts, "headers/prefix.h", "
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct struct_ {
            pub i: ::std::os::raw::c_int,
        }
        impl ::std::default::Default for struct_ {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
        #[derive(Copy, Clone)]
        #[repr(u32)]
        #[derive(Debug)]
        pub enum enum_ { A = 0, B = 1, }
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct union {
            pub _bindgen_data_: [u32; 1usize],
        }
        impl union {
            pub unsafe fn s(&mut self) -> *mut struct_ {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(0))
            }
            pub unsafe fn e(&mut self) -> *mut enum_ {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(0))
            }
        }
        impl ::std::default::Default for union {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
        extern \"C\" {
            #[link_name = \"test_var\"]
            pub static mut var: ::std::os::raw::c_int;
        }
        extern \"C\" {
            #[link_name = \"test_fn\"]
            pub fn fn_() -> ::std::os::raw::c_int;
            #[link_name = \"test_fn2\"]
            pub fn fn2() -> union;
            #[link_name = \"test_fn3\"]
            pub fn fn3() -> struct_;
            #[link_name = \"test_fn4\"]
            pub fn fn4() -> enum_;
        }
    ");
}

#[test]
fn remove_prefix_no_enum() {
    let opts = BindgenOptions {
        remove_prefix: "test_".into(),
        rust_enums: false,
        ..Default::default()
    };
    assert_bind_eq(opts, "headers/prefix.h", "
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct struct_ {
            pub i: ::std::os::raw::c_int,
        }
        impl ::std::default::Default for struct_ {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
        pub type enum_ = u32;
        pub const A: enum_ = 0;
        pub const B: enum_ = 1;
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct union {
            pub _bindgen_data_: [u32; 1usize],
        }
        impl union {
            pub unsafe fn s(&mut self) -> *mut struct_ {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(0))
            }
            pub unsafe fn e(&mut self) -> *mut enum_ {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(0))
            }
        }
        impl ::std::default::Default for union {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
        extern \"C\" {
            #[link_name = \"test_var\"]
            pub static mut var: ::std::os::raw::c_int;
        }
        extern \"C\" {
            #[link_name = \"test_fn\"]
            pub fn fn_() -> ::std::os::raw::c_int;
            #[link_name = \"test_fn2\"]
            pub fn fn2() -> union;
            #[link_name = \"test_fn3\"]
            pub fn fn3() -> struct_;
            #[link_name = \"test_fn4\"]
            pub fn fn4() -> enum_;
        }
    ");
}

#[test]
fn ignore_cyclic_references() {
    let bindings = Builder::new("tests/headers/cyclic.h")
        .remove_prefix("my_")
        .generate()
        .unwrap()
        .to_string();
    
    assert!(!bindings.contains("pub type type_t = type_t;"));
    assert!(!bindings.contains("pub type struct_t = struct_t;"));
    assert!(!bindings.contains("pub type enum_t = enum_t;"));
}
