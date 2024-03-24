#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default)]
pub struct WouldBeCopyButWeAreNotDerivingCopy {
    pub x: ::std::os::raw::c_int,
}
const _: () = {
    [
        "Size of WouldBeCopyButWeAreNotDerivingCopy",
    ][::std::mem::size_of::<WouldBeCopyButWeAreNotDerivingCopy>() - 4usize];
    [
        "Alignment of WouldBeCopyButWeAreNotDerivingCopy",
    ][::std::mem::align_of::<WouldBeCopyButWeAreNotDerivingCopy>() - 4usize];
    [
        "Offset of field: WouldBeCopyButWeAreNotDerivingCopy::x",
    ][::std::mem::offset_of!(WouldBeCopyButWeAreNotDerivingCopy, x) - 0usize];
};
