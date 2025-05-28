#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(transparent)]
pub struct __bindgen_marker_Reference<T: ?Sized>(*const T);
#[repr(transparent)]
pub struct __bindgen_marker_RValueReference<T: ?Sized>(*const T);
unsafe extern "C" {
    #[link_name = "\u{1}_Z17receive_referenceRKi"]
    pub fn receive_reference(
        input: __bindgen_marker_Reference<*const ::std::os::raw::c_int>,
    ) -> __bindgen_marker_Reference<*const ::std::os::raw::c_int>;
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z21receive_mut_referenceRi"]
    pub fn receive_mut_reference(
        input: __bindgen_marker_Reference<*mut ::std::os::raw::c_int>,
    ) -> __bindgen_marker_Reference<*mut ::std::os::raw::c_int>;
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z24receive_rvalue_referenceOi"]
    pub fn receive_rvalue_reference(
        input: __bindgen_marker_RValueReference<*mut ::std::os::raw::c_int>,
    );
}
