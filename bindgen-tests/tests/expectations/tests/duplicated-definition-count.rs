#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct BitStream {
    pub _address: u8,
}
const _: () = {
    ["Size of BitStream"][::std::mem::size_of::<BitStream>() - 1usize];
    ["Alignment of BitStream"][::std::mem::align_of::<BitStream>() - 1usize];
};
extern "C" {
    #[link_name = "\u{1}_ZN9BitStream5WriteEPKcj"]
    pub fn BitStream_Write(
        this: *mut BitStream,
        inputByteArray: *const ::std::os::raw::c_char,
        numberOfBytes: ::std::os::raw::c_uint,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN9BitStream5WriteEPS_j"]
    pub fn BitStream_Write1(
        this: *mut BitStream,
        bitStream: *mut BitStream,
        numberOfBits: ::std::os::raw::c_uint,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN9BitStream6Write1Ev"]
    pub fn BitStream_Write11(this: *mut BitStream);
}
impl BitStream {
    #[inline]
    pub unsafe fn Write(
        &mut self,
        inputByteArray: *const ::std::os::raw::c_char,
        numberOfBytes: ::std::os::raw::c_uint,
    ) {
        BitStream_Write(self, inputByteArray, numberOfBytes)
    }
    #[inline]
    pub unsafe fn Write1(
        &mut self,
        bitStream: *mut BitStream,
        numberOfBits: ::std::os::raw::c_uint,
    ) {
        BitStream_Write1(self, bitStream, numberOfBits)
    }
    #[inline]
    pub unsafe fn Write11(&mut self) {
        BitStream_Write11(self)
    }
}
