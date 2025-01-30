#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
impl MyError {
    pub const MyResultInvalid: MyError = MyError(const {
        core::num::NonZero::new(1).unwrap()
    });
}
impl MyError {
    pub const MyResultAnotherError: MyError = MyError(const {
        core::num::NonZero::new(2).unwrap()
    });
}
pub type MyResult = Result<(), MyError>;
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MyError(pub core::num::NonZero<::std::os::raw::c_uint>);
pub use self::MyResult as AnotherResult;
extern "C" {
    pub fn some_function() -> MyResult;
}
extern "C" {
    pub fn another_function() -> AnotherResult;
}
