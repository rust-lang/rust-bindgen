#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Rooted<T> {
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
        pub member: T,
    }
    impl<T> Default for Rooted<T> {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    pub type AutoValueVector_Alias = ::std::os::raw::c_int;
    #[allow(clippy::unnecessary_operation, clippy::identity_op)]
    const _: () = {
        [
            "Size of template specialization: Rooted_open0_int_close0",
        ][::std::mem::size_of::<root::Rooted<::std::os::raw::c_int>>() - 4usize];
        [
            "Align of template specialization: Rooted_open0_int_close0",
        ][::std::mem::align_of::<root::Rooted<::std::os::raw::c_int>>() - 4usize];
    };
    #[allow(clippy::unnecessary_operation, clippy::identity_op)]
    const _: () = {
        [
            "Size of template specialization: Rooted_open0_AutoValueVector_Alias_close0",
        ][::std::mem::size_of::<root::Rooted<root::AutoValueVector_Alias>>() - 4usize];
        [
            "Align of template specialization: Rooted_open0_AutoValueVector_Alias_close0",
        ][::std::mem::align_of::<root::Rooted<root::AutoValueVector_Alias>>() - 4usize];
    };
}
