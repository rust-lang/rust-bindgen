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
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Foo {
        pub foo: ::std::os::raw::c_int,
    }
    #[test]
    fn bindgen_test_layout_Foo() {
        const UNINIT: ::std::mem::MaybeUninit<Foo> =
            ::std::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::std::mem::size_of::<Foo>(),
            4usize,
            concat!("Size of: ", stringify!(Foo))
        );
        assert_eq!(
            ::std::mem::align_of::<Foo>(),
            4usize,
            concat!("Alignment of ", stringify!(Foo))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).foo) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Foo),
                "::",
                stringify!(foo)
            )
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3Foo7get_fooEv"]
        pub fn Foo_get_foo(this: *mut root::Foo) -> ::std::os::raw::c_int;
    }
    impl Foo {
        #[inline]
        pub unsafe fn get_foo(&mut self) -> ::std::os::raw::c_int {
            Foo_get_foo(self)
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Bar {
        pub bar: ::std::os::raw::c_int,
    }
    #[test]
    fn bindgen_test_layout_Bar() {
        const UNINIT: ::std::mem::MaybeUninit<Bar> =
            ::std::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::std::mem::size_of::<Bar>(),
            4usize,
            concat!("Size of: ", stringify!(Bar))
        );
        assert_eq!(
            ::std::mem::align_of::<Bar>(),
            4usize,
            concat!("Alignment of ", stringify!(Bar))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Bar),
                "::",
                stringify!(bar)
            )
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3Bar7get_barEv"]
        pub fn Bar_get_bar(this: *mut root::Bar) -> ::std::os::raw::c_int;
    }
    impl Bar {
        #[inline]
        pub unsafe fn get_bar(&mut self) -> ::std::os::raw::c_int {
            Bar_get_bar(self)
        }
    }
}
