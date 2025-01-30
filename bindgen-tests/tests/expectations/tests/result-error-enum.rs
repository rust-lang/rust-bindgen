#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
impl MyResultError {
    pub const MyResultErr1: MyResultError = MyResultError(const {
        core::num::NonZero::new(1).unwrap()
    });
}
impl MyResultError {
    pub const MyResultErr2: MyResultError = MyResultError(const {
        core::num::NonZero::new(2).unwrap()
    });
}
pub type MyResult = Result<(), MyResultError>;
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MyResultError(pub core::num::NonZero<::std::os::raw::c_uint>);
pub use self::MyResult as ResultTypedef;
extern "C" {
    pub fn do_something() -> MyResult;
}
extern "C" {
    pub fn do_something2() -> ResultTypedef;
}
