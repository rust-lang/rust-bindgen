#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
    const _: () = {
        ["Size of Point"][::std::mem::size_of::<Point>() - 8usize];
        ["Alignment of Point"][::std::mem::align_of::<Point>() - 4usize];
        ["Offset of field: Point::x"][::std::mem::offset_of!(Point, x) - 0usize];
        ["Offset of field: Point::y"][::std::mem::offset_of!(Point, y) - 4usize];
    };
    const _: () = {
        ["Size of Angle"][::std::mem::size_of::<Angle>() - 8usize];
        ["Alignment of Angle"][::std::mem::align_of::<Angle>() - 4usize];
        ["Offset of field: Angle::a"][::std::mem::offset_of!(Angle, a) - 0usize];
        ["Offset of field: Angle::b"][::std::mem::offset_of!(Angle, b) - 4usize];
    };
    pub const NUMBER: root::number = 42;
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
        const _: () = {
            ["Size of Point"][::std::mem::size_of::<Point>() - 8usize];
            ["Alignment of Point"][::std::mem::align_of::<Point>() - 4usize];
            ["Offset of field: Point::x"][::std::mem::offset_of!(Point, x) - 0usize];
            ["Offset of field: Point::y"][::std::mem::offset_of!(Point, y) - 4usize];
        };
        const _: () = {
            ["Size of Angle"][::std::mem::size_of::<Angle>() - 8usize];
            ["Alignment of Angle"][::std::mem::align_of::<Angle>() - 4usize];
            ["Offset of field: Angle::a"][::std::mem::offset_of!(Angle, a) - 0usize];
            ["Offset of field: Angle::b"][::std::mem::offset_of!(Angle, b) - 4usize];
        };
        pub const NUMBER: root::ns::number = 42;
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
