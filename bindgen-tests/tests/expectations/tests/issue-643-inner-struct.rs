#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct rte_ring {
    pub memzone: *mut rte_memzone,
    pub prod: rte_ring_prod,
    pub cons: rte_ring_cons,
    pub ring: __IncompleteArrayField<*mut ::std::os::raw::c_void>,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct rte_ring_prod {
    pub watermark: ::std::os::raw::c_uint,
}
const _: () = {
    assert!(::std::mem::size_of::<rte_ring_prod>() == 4usize, "Size of rte_ring_prod");
    assert!(
        ::std::mem::align_of::<rte_ring_prod>() == 4usize,
        "Alignment of rte_ring_prod",
    );
    assert!(
        ::std::mem::offset_of!(rte_ring_prod, watermark) == 0usize,
        "Offset of field: rte_ring_prod::watermark",
    );
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct rte_ring_cons {
    pub sc_dequeue: ::std::os::raw::c_uint,
}
const _: () = {
    assert!(::std::mem::size_of::<rte_ring_cons>() == 4usize, "Size of rte_ring_cons");
    assert!(
        ::std::mem::align_of::<rte_ring_cons>() == 4usize,
        "Alignment of rte_ring_cons",
    );
    assert!(
        ::std::mem::offset_of!(rte_ring_cons, sc_dequeue) == 0usize,
        "Offset of field: rte_ring_cons::sc_dequeue",
    );
};
const _: () = {
    assert!(::std::mem::size_of::<rte_ring>() == 16usize, "Size of rte_ring");
    assert!(::std::mem::align_of::<rte_ring>() == 8usize, "Alignment of rte_ring");
    assert!(
        ::std::mem::offset_of!(rte_ring, memzone) == 0usize,
        "Offset of field: rte_ring::memzone",
    );
    assert!(
        ::std::mem::offset_of!(rte_ring, prod) == 8usize,
        "Offset of field: rte_ring::prod",
    );
    assert!(
        ::std::mem::offset_of!(rte_ring, cons) == 12usize,
        "Offset of field: rte_ring::cons",
    );
    assert!(
        ::std::mem::offset_of!(rte_ring, ring) == 16usize,
        "Offset of field: rte_ring::ring",
    );
};
impl Default for rte_ring {
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
pub struct rte_memzone {
    pub _address: u8,
}
