#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const NULL: *mut ::std::os::raw::c_void = ::std::ptr::null_mut();
pub const eNotifyAction_eNoAction: eNotifyAction = 0;
pub const eNotifyAction_eSetBits: eNotifyAction = 1;
pub const eNotifyAction_eIncrement: eNotifyAction = 2;
pub const eNotifyAction_eSetValueWithOverwrite: eNotifyAction = 3;
pub const eNotifyAction_eSetValueWithoutOverwrite: eNotifyAction = 4;
pub type eNotifyAction = ::std::os::raw::c_uint;
pub const tskDEFAULT_INDEX_TO_NOTIFY: u32 = 0;
pub type TaskHandle_t = *mut ::std::os::raw::c_void;
pub type BaseType_t = ::std::os::raw::c_long;
pub type UBaseType_t = ::std::os::raw::c_ulong;
extern "C" {
    pub fn xTaskGenericNotify(
        xTaskToNotify: TaskHandle_t,
        uxIndexToNotify: UBaseType_t,
        ulValue: u32,
        eAction: eNotifyAction,
        pulPreviousNotificationValue: *mut u32,
    ) -> BaseType_t;
}
#[allow(non_snake_case, unused_mut, unsafe_code)]
#[inline(always)]
pub unsafe extern "C" fn xTaskNotifyGive(mut xTaskToNotify: TaskHandle_t) -> BaseType_t {
    xTaskGenericNotify(
        xTaskToNotify,
        0,
        0,
        (eNotifyAction_eIncrement).into(),
        ::std::ptr::null_mut(),
    )
}
