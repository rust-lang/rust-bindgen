#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
impl AVCodecID {
    pub const AV_CODEC_ID_TTF: AVCodecID = AVCodecID::AV_CODEC_ID_FIRST_UNKNOWN;
}
pub type AVCodecID_ctype = ::std::os::raw::c_uint;
pub const AVCodecID_AV_CODEC_ID_FIRST_UNKNOWN: AVCodecID_ctype = 98304;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AVCodecID {
    AV_CODEC_ID_FIRST_UNKNOWN = 98304,
}
