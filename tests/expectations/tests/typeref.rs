#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct mozilla_FragmentOrURL {
    pub mIsLocalRef: bool,
}
#[test]
fn bindgen_test_layout_mozilla_FragmentOrURL() {
    assert_eq!(
        ::std::mem::size_of::<mozilla_FragmentOrURL>(),
        1usize,
        concat!("Size of: ", stringify!(mozilla_FragmentOrURL))
    );
    assert_eq!(
        ::std::mem::align_of::<mozilla_FragmentOrURL>(),
        1usize,
        concat!("Alignment of ", stringify!(mozilla_FragmentOrURL))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<mozilla_FragmentOrURL>() };
            let struct_ptr = &struct_instance as *const mozilla_FragmentOrURL;
            let field_ptr = std::ptr::addr_of!(struct_instance.mIsLocalRef);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(mozilla_FragmentOrURL),
            "::",
            stringify!(mIsLocalRef)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct mozilla_Position {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_mozilla_Position() {
    assert_eq!(
        ::std::mem::size_of::<mozilla_Position>(),
        1usize,
        concat!("Size of: ", stringify!(mozilla_Position))
    );
    assert_eq!(
        ::std::mem::align_of::<mozilla_Position>(),
        1usize,
        concat!("Alignment of ", stringify!(mozilla_Position))
    );
}
#[repr(C)]
pub struct mozilla_StyleShapeSource {
    pub __bindgen_anon_1: mozilla_StyleShapeSource__bindgen_ty_1,
}
#[repr(C)]
pub union mozilla_StyleShapeSource__bindgen_ty_1 {
    pub mPosition: *mut mozilla_Position,
    pub mFragmentOrURL: *mut mozilla_FragmentOrURL,
}
impl Default for mozilla_StyleShapeSource__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for mozilla_StyleShapeSource {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Bar {
    pub mFoo: *mut nsFoo,
}
#[test]
fn bindgen_test_layout_Bar() {
    assert_eq!(
        ::std::mem::size_of::<Bar>(),
        8usize,
        concat!("Size of: ", stringify!(Bar))
    );
    assert_eq!(
        ::std::mem::align_of::<Bar>(),
        8usize,
        concat!("Alignment of ", stringify!(Bar))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Bar>() };
            let struct_ptr = &struct_instance as *const Bar;
            let field_ptr = std::ptr::addr_of!(struct_instance.mFoo);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(Bar), "::", stringify!(mFoo))
    );
}
impl Default for Bar {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct nsFoo {
    pub mBar: mozilla_StyleShapeSource,
}
#[test]
fn bindgen_test_layout_nsFoo() {
    assert_eq!(
        ::std::mem::size_of::<nsFoo>(),
        8usize,
        concat!("Size of: ", stringify!(nsFoo))
    );
    assert_eq!(
        ::std::mem::align_of::<nsFoo>(),
        8usize,
        concat!("Alignment of ", stringify!(nsFoo))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<nsFoo>() };
            let struct_ptr = &struct_instance as *const nsFoo;
            let field_ptr = std::ptr::addr_of!(struct_instance.mBar);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nsFoo),
            "::",
            stringify!(mBar)
        )
    );
}
impl Default for nsFoo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn __bindgen_test_layout_mozilla_StyleShapeSource_open0_int_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<mozilla_StyleShapeSource>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(mozilla_StyleShapeSource)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<mozilla_StyleShapeSource>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(mozilla_StyleShapeSource)
        )
    );
}
