#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default)]
pub struct WouldBeCopyButWeAreNotDerivingCopy {
    pub x: ::std::os::raw::c_int,
}
const _: () = {
    assert!(
        ::std::mem::size_of::<WouldBeCopyButWeAreNotDerivingCopy>() == 4usize,
        "Size of WouldBeCopyButWeAreNotDerivingCopy",
    );
    assert!(
        ::std::mem::align_of::<WouldBeCopyButWeAreNotDerivingCopy>() == 4usize,
        "Alignment of WouldBeCopyButWeAreNotDerivingCopy",
    );
    assert!(
        ::std::mem::offset_of!(WouldBeCopyButWeAreNotDerivingCopy, x) == 0usize,
        "Offset of field: WouldBeCopyButWeAreNotDerivingCopy::x",
    );
};
