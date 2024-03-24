#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Point {
        pub x: ::std::os::raw::c_int,
    }
    const _: () = {
        ["Size of Point"][::std::mem::size_of::<Point>() - 4usize];
        ["Alignment of Point"][::std::mem::align_of::<Point>() - 4usize];
        ["Offset of field: Point::x"][::std::mem::offset_of!(Point, x) - 0usize];
    };
    pub mod ns {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Point {
            pub x: ::std::os::raw::c_int,
        }
        const _: () = {
            ["Size of Point"][::std::mem::size_of::<Point>() - 4usize];
            ["Alignment of Point"][::std::mem::align_of::<Point>() - 4usize];
            ["Offset of field: Point::x"][::std::mem::offset_of!(Point, x) - 0usize];
        };
        extern "C" {
            #[link_name = "\u{1}_ZN2ns3fooEv"]
            pub fn foo() -> ::std::os::raw::c_int;
            #[link_name = "\u{1}_ZN2ns3barEv"]
            pub fn bar() -> ::std::os::raw::c_int;
        }
    }
    extern "C" {
        #[link_name = "\u{1}_Z3foov"]
        pub fn foo() -> ::std::os::raw::c_int;
        #[link_name = "\u{1}_Z3barv"]
        pub fn bar() -> ::std::os::raw::c_int;
    }
}
