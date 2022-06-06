#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct i {
        pub j: *mut root::i,
        pub k: *mut root::i,
        pub l: bool,
    }
    #[test]
    fn bindgen_test_layout_i() {
        assert_eq!(
            ::std::mem::size_of::<i>(),
            24usize,
            concat!("Size of: ", stringify!(i))
        );
        assert_eq!(
            ::std::mem::align_of::<i>(),
            8usize,
            concat!("Alignment of ", stringify!(i))
        );
        fn test_field_j() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<i>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).j) as usize - ptr as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(i),
                    "::",
                    stringify!(j)
                )
            );
        }
        test_field_j();
        fn test_field_k() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<i>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).k) as usize - ptr as usize
                },
                8usize,
                concat!(
                    "Offset of field: ",
                    stringify!(i),
                    "::",
                    stringify!(k)
                )
            );
        }
        test_field_k();
        fn test_field_l() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<i>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).l) as usize - ptr as usize
                },
                16usize,
                concat!(
                    "Offset of field: ",
                    stringify!(i),
                    "::",
                    stringify!(l)
                )
            );
        }
        test_field_l();
    }
    impl Default for i {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct d {
        pub m: root::i,
    }
    #[test]
    fn bindgen_test_layout_d() {
        assert_eq!(
            ::std::mem::size_of::<d>(),
            24usize,
            concat!("Size of: ", stringify!(d))
        );
        assert_eq!(
            ::std::mem::align_of::<d>(),
            8usize,
            concat!("Alignment of ", stringify!(d))
        );
        fn test_field_m() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<d>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).m) as usize - ptr as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(d),
                    "::",
                    stringify!(m)
                )
            );
        }
        test_field_m();
    }
    impl Default for d {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(u32)]
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    pub enum n {
        o = 0,
        p = 1,
        q = 2,
        r = 3,
        s = 4,
        t = 5,
        b = 6,
        ae = 7,
        e = 8,
        ag = 9,
        ah = 10,
        ai = 11,
    }
    #[repr(C)]
    pub struct F {
        pub w: [u64; 33usize],
    }
    #[test]
    fn bindgen_test_layout_F() {
        assert_eq!(
            ::std::mem::size_of::<F>(),
            264usize,
            concat!("Size of: ", stringify!(F))
        );
        assert_eq!(
            ::std::mem::align_of::<F>(),
            8usize,
            concat!("Alignment of ", stringify!(F))
        );
        fn test_field_w() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<F>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).w) as usize - ptr as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(F),
                    "::",
                    stringify!(w)
                )
            );
        }
        test_field_w();
    }
    impl Default for F {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
}
