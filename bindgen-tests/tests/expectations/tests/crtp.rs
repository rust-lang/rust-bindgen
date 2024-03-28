#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Base {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Derived {
    pub _address: u8,
}
const _: () = {
    assert!(::std::mem::size_of::<Derived>() == 1usize, "Size of Derived");
    assert!(::std::mem::align_of::<Derived>() == 1usize, "Alignment of Derived");
};
#[repr(C)]
#[derive(Debug, Default)]
pub struct BaseWithDestructor {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct DerivedFromBaseWithDestructor {
    pub _address: u8,
}
const _: () = {
    assert!(
        ::std::mem::size_of::<DerivedFromBaseWithDestructor>() == 1usize,
        "Size of DerivedFromBaseWithDestructor",
    );
    assert!(
        ::std::mem::align_of::<DerivedFromBaseWithDestructor>() == 1usize,
        "Alignment of DerivedFromBaseWithDestructor",
    );
};
const _: () = {
    assert!(
        ::std::mem::size_of::<Base>() == 1usize,
        "Size of template specialization: Base_open0_Derived_close0",
    );
    assert!(
        ::std::mem::align_of::<Base>() == 1usize,
        "Align of template specialization: Base_open0_Derived_close0",
    );
};
const _: () = {
    assert!(
        ::std::mem::size_of::<BaseWithDestructor>() == 1usize,
        "Size of template specialization: BaseWithDestructor_open0_DerivedFromBaseWithDestructor_close0",
    );
    assert!(
        ::std::mem::align_of::<BaseWithDestructor>() == 1usize,
        "Align of template specialization: BaseWithDestructor_open0_DerivedFromBaseWithDestructor_close0",
    );
};
