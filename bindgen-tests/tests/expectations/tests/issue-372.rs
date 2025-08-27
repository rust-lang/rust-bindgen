#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    /// If Bindgen could only determine the size and alignment of a
    /// type, it is represented like this.
    #[derive(PartialEq, Copy, Clone, Debug, Hash)]
    #[repr(C)]
    pub struct __BindgenOpaqueArray<T: Copy, const N: usize>(pub [T; N]);
    impl<T: Copy + Default, const N: usize> Default for __BindgenOpaqueArray<T, N> {
        fn default() -> Self {
            Self([<T as Default>::default(); N])
        }
    }
    #[allow(unused_imports)]
    use self::super::root;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct i {
        pub j: *mut root::i,
        pub k: *mut root::i,
        pub l: bool,
    }
    #[allow(clippy::unnecessary_operation, clippy::identity_op)]
    const _: () = {
        ["Size of i"][::std::mem::size_of::<i>() - 24usize];
        ["Alignment of i"][::std::mem::align_of::<i>() - 8usize];
        ["Offset of field: i::j"][::std::mem::offset_of!(i, j) - 0usize];
        ["Offset of field: i::k"][::std::mem::offset_of!(i, k) - 8usize];
        ["Offset of field: i::l"][::std::mem::offset_of!(i, l) - 16usize];
    };
    impl Default for i {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct d {
        pub m: root::i,
    }
    #[allow(clippy::unnecessary_operation, clippy::identity_op)]
    const _: () = {
        ["Size of d"][::std::mem::size_of::<d>() - 24usize];
        ["Alignment of d"][::std::mem::align_of::<d>() - 8usize];
        ["Offset of field: d::m"][::std::mem::offset_of!(d, m) - 0usize];
    };
    impl Default for d {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(u32)]
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    pub enum n {
        o = 0,
        p = 1,
        q = 2,
        r = 3,
        s = 4,
        t = 5,
        b = 6,
        ae = 7,
        e = 8,
        ag = 9,
        ah = 10,
        ai = 11,
    }
    #[repr(C)]
    pub struct F {
        pub w: root::__BindgenOpaqueArray<u64, 33usize>,
    }
    #[allow(clippy::unnecessary_operation, clippy::identity_op)]
    const _: () = {
        ["Size of F"][::std::mem::size_of::<F>() - 264usize];
        ["Alignment of F"][::std::mem::align_of::<F>() - 8usize];
        ["Offset of field: F::w"][::std::mem::offset_of!(F, w) - 0usize];
    };
    impl Default for F {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
}
