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
    extern "C" {
        #[link_name = "\u{1}_Z9top_levelv"]
        pub fn top_level();
    }
    pub mod whatever {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub type whatever_other_thing_t = whatever_int_t;
        pub type whatever_int_t = ::std::os::raw::c_int;
        extern "C" {
            #[link_name = "\u{1}_ZN8whatever11in_whateverEv"]
            pub fn in_whatever();
        }
    }
    pub mod _bindgen_mod_id_17 {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct A {
            pub b: root::whatever::whatever_int_t,
        }
        #[test]
        fn bindgen_test_layout_A() {
            assert_eq!(
                ::std::mem::size_of::<A>(),
                4usize,
                concat!("Size of: ", stringify!(A))
            );
            assert_eq!(
                ::std::mem::align_of::<A>(),
                4usize,
                concat!("Alignment of ", stringify!(A))
            );
            assert_eq!(
                unsafe { &(*(::std::ptr::null::<A>())).b as *const _ as usize },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(A),
                    "::",
                    stringify!(b)
                )
            );
        }
    }
    #[repr(C)]
    #[derive(Debug)]
    pub struct C<T> {
        pub _base: root::_bindgen_mod_id_17::A,
        pub m_c: T,
        pub m_c_ptr: *mut T,
        pub m_c_arr: [T; 10usize],
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    }
    impl<T> Default for C<T> {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    pub mod w {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub type whatever_int_t = ::std::os::raw::c_uint;
        #[repr(C)]
        #[derive(Debug)]
        pub struct D<T> {
            pub m_c: root::C<T>,
            pub _phantom_0:
                ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
        }
        impl<T> Default for D<T> {
            fn default() -> Self {
                let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
                unsafe {
                    ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                    s.assume_init()
                }
            }
        }
        extern "C" {
            #[link_name = "\u{1}_ZN1w3hehEv"]
            pub fn heh() -> root::w::whatever_int_t;
        }
        extern "C" {
            #[link_name = "\u{1}_ZN1w3fooEv"]
            pub fn foo() -> root::C<::std::os::raw::c_int>;
        }
        extern "C" {
            #[link_name = "\u{1}_ZN1w4barrEv"]
            pub fn barr() -> root::C<f32>;
        }
    }
    pub mod foobar {
        #[allow(unused_imports)]
        use self::super::super::root;
        extern "C" {
            #[link_name = "\u{1}_ZN6foobar3fooEv"]
            pub fn foo();
        }
    }
    pub mod faraway {
        #[allow(unused_imports)]
        use self::super::super::root;
        extern "C" {
            #[link_name = "\u{1}_ZN7faraway3barEv"]
            pub fn bar();
        }
    }
}
