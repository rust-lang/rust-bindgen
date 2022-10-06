#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    pub type number = ::std::os::raw::c_int;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Point {
        pub x: root::number,
        pub y: root::number,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Angle {
        pub a: root::number,
        pub b: root::number,
    }
    pub const NUMBER: root::number = 42;
    #[test]
    fn bindgen_test_layout_Point() {
        const UNINIT: ::std::mem::MaybeUninit<Point> =
            ::std::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::std::mem::size_of::<Point>(),
            8usize,
            concat!("Size of: ", stringify!(Point))
        );
        assert_eq!(
            ::std::mem::align_of::<Point>(),
            4usize,
            concat!("Alignment of ", stringify!(Point))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Point),
                "::",
                stringify!(x)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(Point),
                "::",
                stringify!(y)
            )
        );
    }
    #[test]
    fn bindgen_test_layout_Angle() {
        const UNINIT: ::std::mem::MaybeUninit<Angle> =
            ::std::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::std::mem::size_of::<Angle>(),
            8usize,
            concat!("Size of: ", stringify!(Angle))
        );
        assert_eq!(
            ::std::mem::align_of::<Angle>(),
            4usize,
            concat!("Alignment of ", stringify!(Angle))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Angle),
                "::",
                stringify!(a)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(Angle),
                "::",
                stringify!(b)
            )
        );
    }
    pub mod ns {
        pub type number = ::std::os::raw::c_int;
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Point {
            pub x: root::ns::number,
            pub y: root::ns::number,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Angle {
            pub a: root::ns::number,
            pub b: root::ns::number,
        }
        pub const NUMBER: root::ns::number = 42;
        #[test]
        fn bindgen_test_layout_Point() {
            const UNINIT: ::std::mem::MaybeUninit<Point> =
                ::std::mem::MaybeUninit::uninit();
            let ptr = UNINIT.as_ptr();
            assert_eq!(
                ::std::mem::size_of::<Point>(),
                8usize,
                concat!("Size of: ", stringify!(Point))
            );
            assert_eq!(
                ::std::mem::align_of::<Point>(),
                4usize,
                concat!("Alignment of ", stringify!(Point))
            );
            assert_eq!(
                unsafe {
                    ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(Point),
                    "::",
                    stringify!(x)
                )
            );
            assert_eq!(
                unsafe {
                    ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize
                },
                4usize,
                concat!(
                    "Offset of field: ",
                    stringify!(Point),
                    "::",
                    stringify!(y)
                )
            );
        }
        #[test]
        fn bindgen_test_layout_Angle() {
            const UNINIT: ::std::mem::MaybeUninit<Angle> =
                ::std::mem::MaybeUninit::uninit();
            let ptr = UNINIT.as_ptr();
            assert_eq!(
                ::std::mem::size_of::<Angle>(),
                8usize,
                concat!("Size of: ", stringify!(Angle))
            );
            assert_eq!(
                ::std::mem::align_of::<Angle>(),
                4usize,
                concat!("Alignment of ", stringify!(Angle))
            );
            assert_eq!(
                unsafe {
                    ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(Angle),
                    "::",
                    stringify!(a)
                )
            );
            assert_eq!(
                unsafe {
                    ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize
                },
                4usize,
                concat!(
                    "Offset of field: ",
                    stringify!(Angle),
                    "::",
                    stringify!(b)
                )
            );
        }
        #[allow(unused_imports)]
        use self::super::super::root;
        extern "C" {
            #[link_name = "\u{1}_ZN2ns3fooEv"]
            pub fn foo() -> ::std::os::raw::c_int;
        }
        extern "C" {
            #[link_name = "\u{1}_ZN2ns3barEi"]
            pub fn bar(x: root::ns::number) -> ::std::os::raw::c_int;
        }
        extern "C" {
            #[link_name = "\u{1}_ZN2ns3bazENS_5PointE"]
            pub fn baz(point: root::ns::Point) -> ::std::os::raw::c_int;
        }
    }
    #[allow(unused_imports)]
    use self::super::root;
    extern "C" {
        #[link_name = "\u{1}_Z3foov"]
        pub fn foo() -> ::std::os::raw::c_int;
    }
    extern "C" {
        #[link_name = "\u{1}_Z3bari"]
        pub fn bar(x: root::number) -> ::std::os::raw::c_int;
    }
    extern "C" {
        #[link_name = "\u{1}_Z3baz5Point"]
        pub fn baz(point: root::Point) -> ::std::os::raw::c_int;
    }
}
