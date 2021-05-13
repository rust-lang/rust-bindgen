#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct InnerType {
    pub foo: ::std::os::raw::c_long,
}
pub type InnerType_related_type = ::std::os::raw::c_int;
impl __bindgen_has_inner_type_related_type for InnerType {
    type related_type = InnerType_related_type;
}
#[test]
fn bindgen_test_layout_InnerType() {
    assert_eq!(
        ::std::mem::size_of::<InnerType>(),
        8usize,
        concat!("Size of: ", stringify!(InnerType))
    );
    assert_eq!(
        ::std::mem::align_of::<InnerType>(),
        8usize,
        concat!("Alignment of ", stringify!(InnerType))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<InnerType>())).foo as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(InnerType),
            "::",
            stringify!(foo)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct InnerType2 {
    pub foo: ::std::os::raw::c_long,
}
pub type InnerType2_related_type = f32;
impl __bindgen_has_inner_type_related_type for InnerType2 {
    type related_type = InnerType2_related_type;
}
#[test]
fn bindgen_test_layout_InnerType2() {
    assert_eq!(
        ::std::mem::size_of::<InnerType2>(),
        8usize,
        concat!("Size of: ", stringify!(InnerType2))
    );
    assert_eq!(
        ::std::mem::align_of::<InnerType2>(),
        8usize,
        concat!("Alignment of ", stringify!(InnerType2))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<InnerType2>())).foo as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(InnerType2),
            "::",
            stringify!(foo)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Container<ContainedType>
where
    ContainedType: __bindgen_has_inner_type_related_type,
{
    pub contents_:
        <ContainedType as __bindgen_has_inner_type_related_type>::related_type,
    pub _phantom_0:
        ::std::marker::PhantomData<::std::cell::UnsafeCell<ContainedType>>,
}
pub type Concrete = Container<InnerType>;
pub type Concrete2 = Container<InnerType2>;
pub trait __bindgen_has_inner_type_related_type {
    type related_type: std::fmt::Debug + Default + Copy + Clone;
}
