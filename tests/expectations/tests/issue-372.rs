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
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<i>())).j as *const _ as usize },
            0usize,
            concat!("Offset of field: ", stringify!(i), "::", stringify!(j))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<i>())).k as *const _ as usize },
            8usize,
            concat!("Offset of field: ", stringify!(i), "::", stringify!(k))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<i>())).l as *const _ as usize },
            16usize,
            concat!("Offset of field: ", stringify!(i), "::", stringify!(l))
        );
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
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<d>())).m as *const _ as usize },
            0usize,
            concat!("Offset of field: ", stringify!(d), "::", stringify!(m))
        );
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
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<F>())).w as *const _ as usize },
            0usize,
            concat!("Offset of field: ", stringify!(F), "::", stringify!(w))
        );
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
