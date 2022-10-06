#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
pub struct Base__bindgen_vtable {
    pub Base_AsDerived: unsafe extern "C" fn(this: *mut Base) -> *mut Derived,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Base {
    pub vtable_: *const Base__bindgen_vtable,
}
#[test]
fn bindgen_test_layout_Base() {
    assert_eq!(
        ::std::mem::size_of::<Base>(),
        8usize,
        concat!("Size of: ", stringify!(Base))
    );
    assert_eq!(
        ::std::mem::align_of::<Base>(),
        8usize,
        concat!("Alignment of ", stringify!(Base))
    );
}
impl Default for Base {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    #[link_name = "\u{1}_ZN4Base9AsDerivedEv"]
    pub fn Base_AsDerived(this: *mut ::std::os::raw::c_void) -> *mut Derived;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Derived {
    pub _base: Base,
}
#[test]
fn bindgen_test_layout_Derived() {
    assert_eq!(
        ::std::mem::size_of::<Derived>(),
        8usize,
        concat!("Size of: ", stringify!(Derived))
    );
    assert_eq!(
        ::std::mem::align_of::<Derived>(),
        8usize,
        concat!("Alignment of ", stringify!(Derived))
    );
}
impl Default for Derived {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
