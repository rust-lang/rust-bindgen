#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Copy, Clone)]
pub union Union {
    pub bytes: [::std::os::raw::c_uchar; 4usize],
    pub word: ::std::os::raw::c_uint,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Union"][::std::mem::size_of::<Union>() - 4usize];
    ["Alignment of Union"][::std::mem::align_of::<Union>() - 4usize];
    ["Offset of field: Union::bytes"][::std::mem::offset_of!(Union, bytes) - 0usize];
    ["Offset of field: Union::word"][::std::mem::offset_of!(Union, word) - 0usize];
};
impl Default for Union {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for Union {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "Union {{ union }}")
    }
}
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct UnionAlias(pub Union);
impl ::std::ops::Deref for UnionAlias {
    type Target = Union;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::std::ops::DerefMut for UnionAlias {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl ::std::fmt::Debug for UnionAlias {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_tuple(stringify!(UnionAlias)).field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructContainingUnionAlias {
    pub ua: UnionAlias,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of StructContainingUnionAlias",
    ][::std::mem::size_of::<StructContainingUnionAlias>() - 4usize];
    [
        "Alignment of StructContainingUnionAlias",
    ][::std::mem::align_of::<StructContainingUnionAlias>() - 4usize];
    [
        "Offset of field: StructContainingUnionAlias::ua",
    ][::std::mem::offset_of!(StructContainingUnionAlias, ua) - 0usize];
};
impl Default for StructContainingUnionAlias {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for StructContainingUnionAlias {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "StructContainingUnionAlias {{ ua: {:?} }}", self.ua)
    }
}
