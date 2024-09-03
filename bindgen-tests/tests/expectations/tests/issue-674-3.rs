#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct nsRefPtrHashtable {
        pub _address: u8,
    }
    pub type nsRefPtrHashtable_UserDataType<PtrType> = *mut PtrType;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct a {
        pub b: u8,
    }
    #[allow(clippy::unnecessary_operation, clippy::identity_op)]
    const _: () = {
        ["Size of a"][::std::mem::size_of::<a>() - 1usize];
        ["Alignment of a"][::std::mem::align_of::<a>() - 1usize];
        ["Offset of field: a::b"][::std::mem::offset_of!(a, b) - 0usize];
    };
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct nsCSSValue {
        pub c: root::a,
    }
    #[allow(clippy::unnecessary_operation, clippy::identity_op)]
    const _: () = {
        ["Size of nsCSSValue"][::std::mem::size_of::<nsCSSValue>() - 1usize];
        ["Alignment of nsCSSValue"][::std::mem::align_of::<nsCSSValue>() - 1usize];
        [
            "Offset of field: nsCSSValue::c",
        ][::std::mem::offset_of!(nsCSSValue, c) - 0usize];
    };
}
