#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Entry<K, V> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<K>>,
    pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<V>>,
    pub _base: K,
    pub mData: V,
}
impl<K, V> Default for Entry<K, V> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct nsBaseHashtable {
    pub _address: u8,
}
pub type nsBaseHashtable_EntryType<K, V> = Entry<K, V>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nsBaseHashtable_EntryPtr<K, V> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<K>>,
    pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<V>>,
    pub mEntry: *mut nsBaseHashtable_EntryType<K, V>,
    pub mExistingEntry: bool,
}
impl<K, V> Default for nsBaseHashtable_EntryPtr<K, V> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
