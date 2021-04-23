#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct InnerType {
    pub foo: ::std::os::raw::c_int,
    pub foo2: ::std::os::raw::c_int,
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
        4usize,
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
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<InnerType>())).foo2 as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(InnerType),
            "::",
            stringify!(foo2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Container<ContainedType>
where
    ContainedType: __bindgen_has_inner_type_related_type,
{
    pub contents_: Container_content_ty<ContainedType>,
    pub _phantom_0:
        ::std::marker::PhantomData<::std::cell::UnsafeCell<ContainedType>>,
}
pub type Container_content_ty<ContainedType> =
    <ContainedType as __bindgen_has_inner_type_related_type>::related_type;
pub type Concrete = Container<InnerType>;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct LaterContainingType {
    pub outer_contents: Concrete,
}
#[test]
fn bindgen_test_layout_LaterContainingType() {
    assert_eq!(
        ::std::mem::size_of::<LaterContainingType>(),
        4usize,
        concat!("Size of: ", stringify!(LaterContainingType))
    );
    assert_eq!(
        ::std::mem::align_of::<LaterContainingType>(),
        4usize,
        concat!("Alignment of ", stringify!(LaterContainingType))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<LaterContainingType>())).outer_contents
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(LaterContainingType),
            "::",
            stringify!(outer_contents)
        )
    );
}
pub trait __bindgen_has_inner_type_related_type {
    type related_type: std::fmt::Debug + Default + Copy + Clone;
}
#[test]
fn __bindgen_test_layout_Container_open0_InnerType_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<Container<InnerType>>(),
        4usize,
        concat!(
            "Size of template specialization: ",
            stringify!(Container<InnerType>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<Container<InnerType>>(),
        4usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(Container<InnerType>)
        )
    );
}
