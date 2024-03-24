#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// This should get an `_address` byte.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Empty {
    pub _address: u8,
}
const _: () = {
    assert!(::std::mem::size_of::<Empty>() == 1usize, "Size of Empty");
    assert!(::std::mem::align_of::<Empty>() == 1usize, "Alignment of Empty");
};
/** This should not get an `_address` byte, so `sizeof(Inherits)` should be
 `1`.*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Inherits {
    pub b: bool,
}
const _: () = {
    assert!(::std::mem::size_of::<Inherits>() == 1usize, "Size of Inherits");
    assert!(::std::mem::align_of::<Inherits>() == 1usize, "Alignment of Inherits");
    assert!(
        ::std::mem::offset_of!(Inherits, b) == 0usize,
        "Offset of field: Inherits::b",
    );
};
/** This should not get an `_address` byte, but contains `Empty` which *does* get
 one, so `sizeof(Contains)` should be `1 + 1`.*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Contains {
    pub empty: Empty,
    pub b: bool,
}
const _: () = {
    assert!(::std::mem::size_of::<Contains>() == 2usize, "Size of Contains");
    assert!(::std::mem::align_of::<Contains>() == 1usize, "Alignment of Contains");
    assert!(
        ::std::mem::offset_of!(Contains, empty) == 0usize,
        "Offset of field: Contains::empty",
    );
    assert!(
        ::std::mem::offset_of!(Contains, b) == 1usize,
        "Offset of field: Contains::b",
    );
};
