#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct HasPrivate {
    pub mNotPrivate: ::std::os::raw::c_int,
    /// <div rustbindgen private></div>
    mIsPrivate: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<HasPrivate>() == 8usize, "Size of HasPrivate");
    assert!(::std::mem::align_of::<HasPrivate>() == 4usize, "Alignment of HasPrivate");
    assert!(
        ::std::mem::offset_of!(HasPrivate, mNotPrivate) == 0usize,
        "Offset of field: HasPrivate::mNotPrivate",
    );
    assert!(
        ::std::mem::offset_of!(HasPrivate, mIsPrivate) == 4usize,
        "Offset of field: HasPrivate::mIsPrivate",
    );
};
/// <div rustbindgen private></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct VeryPrivate {
    mIsPrivate: ::std::os::raw::c_int,
    mIsAlsoPrivate: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<VeryPrivate>() == 8usize, "Size of VeryPrivate");
    assert!(::std::mem::align_of::<VeryPrivate>() == 4usize, "Alignment of VeryPrivate");
    assert!(
        ::std::mem::offset_of!(VeryPrivate, mIsPrivate) == 0usize,
        "Offset of field: VeryPrivate::mIsPrivate",
    );
    assert!(
        ::std::mem::offset_of!(VeryPrivate, mIsAlsoPrivate) == 4usize,
        "Offset of field: VeryPrivate::mIsAlsoPrivate",
    );
};
/// <div rustbindgen private></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ContradictPrivate {
    /// <div rustbindgen private="false"></div>
    pub mNotPrivate: ::std::os::raw::c_int,
    mIsPrivate: ::std::os::raw::c_int,
}
const _: () = {
    assert!(
        ::std::mem::size_of::<ContradictPrivate>() == 8usize,
        "Size of ContradictPrivate",
    );
    assert!(
        ::std::mem::align_of::<ContradictPrivate>() == 4usize,
        "Alignment of ContradictPrivate",
    );
    assert!(
        ::std::mem::offset_of!(ContradictPrivate, mNotPrivate) == 0usize,
        "Offset of field: ContradictPrivate::mNotPrivate",
    );
    assert!(
        ::std::mem::offset_of!(ContradictPrivate, mIsPrivate) == 4usize,
        "Offset of field: ContradictPrivate::mIsPrivate",
    );
};
