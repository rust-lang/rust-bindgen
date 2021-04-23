#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct InnerTypeA {
    pub foo: ::std::os::raw::c_int,
    pub foo2: ::std::os::raw::c_int,
}
pub type InnerTypeA_related_type = ::std::os::raw::c_int;
impl __bindgen_has_inner_type_related_type for InnerTypeA {
    type related_type = InnerTypeA_related_type;
}
#[test]
fn bindgen_test_layout_InnerTypeA() {
    assert_eq!(
        ::std::mem::size_of::<InnerTypeA>(),
        8usize,
        concat!("Size of: ", stringify!(InnerTypeA))
    );
    assert_eq!(
        ::std::mem::align_of::<InnerTypeA>(),
        4usize,
        concat!("Alignment of ", stringify!(InnerTypeA))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<InnerTypeA>())).foo as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(InnerTypeA),
            "::",
            stringify!(foo)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<InnerTypeA>())).foo2 as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(InnerTypeA),
            "::",
            stringify!(foo2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct InnerTypeB {
    pub bar: ::std::os::raw::c_ulong,
}
pub type InnerTypeB_related_type = ::std::os::raw::c_char;
impl __bindgen_has_inner_type_related_type for InnerTypeB {
    type related_type = InnerTypeB_related_type;
}
#[test]
fn bindgen_test_layout_InnerTypeB() {
    assert_eq!(
        ::std::mem::size_of::<InnerTypeB>(),
        8usize,
        concat!("Size of: ", stringify!(InnerTypeB))
    );
    assert_eq!(
        ::std::mem::align_of::<InnerTypeB>(),
        8usize,
        concat!("Alignment of ", stringify!(InnerTypeB))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<InnerTypeB>())).bar as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(InnerTypeB),
            "::",
            stringify!(bar)
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
pub type Concrete = Container<InnerTypeA>;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct LaterContainingType {
    pub outer_contents_a: Concrete,
    pub outer_contents_b: Container<InnerTypeB>,
}
#[test]
fn bindgen_test_layout_LaterContainingType() {
    assert_eq!(
        ::std::mem::size_of::<LaterContainingType>(),
        8usize,
        concat!("Size of: ", stringify!(LaterContainingType))
    );
    assert_eq!(
        ::std::mem::align_of::<LaterContainingType>(),
        4usize,
        concat!("Alignment of ", stringify!(LaterContainingType))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<LaterContainingType>())).outer_contents_a
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(LaterContainingType),
            "::",
            stringify!(outer_contents_a)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<LaterContainingType>())).outer_contents_b
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(LaterContainingType),
            "::",
            stringify!(outer_contents_b)
        )
    );
}
pub trait __bindgen_has_inner_type_related_type {
    type related_type: std::fmt::Debug + Default + Copy + Clone;
}
#[test]
fn __bindgen_test_layout_Container_open0_InnerTypeA_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<Container<InnerTypeA>>(),
        4usize,
        concat!(
            "Size of template specialization: ",
            stringify!(Container<InnerTypeA>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<Container<InnerTypeA>>(),
        4usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(Container<InnerTypeA>)
        )
    );
}
#[test]
fn __bindgen_test_layout_Container_open0_InnerTypeB_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<Container<InnerTypeB>>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(Container<InnerTypeB>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<Container<InnerTypeB>>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(Container<InnerTypeB>)
        )
    );
}
