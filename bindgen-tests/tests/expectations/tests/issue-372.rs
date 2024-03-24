#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
        const UNINIT: ::std::mem::MaybeUninit<i> = ::std::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(::std::mem::size_of::<i>(), 24usize, "Size of i");
        assert_eq!(::std::mem::align_of::<i>(), 8usize, "Alignment of i");
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).j) as usize - ptr as usize },
            0usize,
            "Offset of field: i::j",
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).k) as usize - ptr as usize },
            8usize,
            "Offset of field: i::k",
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).l) as usize - ptr as usize },
            16usize,
            "Offset of field: i::l",
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
        const UNINIT: ::std::mem::MaybeUninit<d> = ::std::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(::std::mem::size_of::<d>(), 24usize, "Size of d");
        assert_eq!(::std::mem::align_of::<d>(), 8usize, "Alignment of d");
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).m) as usize - ptr as usize },
            0usize,
            "Offset of field: d::m",
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
        const UNINIT: ::std::mem::MaybeUninit<F> = ::std::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(::std::mem::size_of::<F>(), 264usize, "Size of F");
        assert_eq!(::std::mem::align_of::<F>(), 8usize, "Alignment of F");
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).w) as usize - ptr as usize },
            0usize,
            "Offset of field: F::w",
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
