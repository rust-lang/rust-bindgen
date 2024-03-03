#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod JS {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub mod detail {
            #[allow(unused_imports)]
            use self::super::super::super::root;
            pub type Wrapped<T> = T;
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct Rooted<T> {
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
            pub ptr: root::JS::detail::Wrapped<T>,
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
    }
}
