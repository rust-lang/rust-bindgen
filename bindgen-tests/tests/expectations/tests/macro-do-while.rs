#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
extern "C" {
    pub fn xTaskDelayUntil(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[allow(non_snake_case, unused_mut, unsafe_code)]
#[inline(always)]
pub unsafe extern "C" fn vTaskDelayUntil(
    mut pxPreviousWakeTime: ::std::os::raw::c_int,
    mut xTimeIncrement: ::std::os::raw::c_int,
) {
    loop {
        {
            let _ = xTaskDelayUntil(pxPreviousWakeTime, xTimeIncrement);
        };
        if 0 != 0 {
            continue;
        }
        break;
    };
}
