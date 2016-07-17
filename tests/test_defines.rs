use bindgen::BindgenOptions;
use support::assert_bind_eq;

#[test]
fn extern_defines() {
    let opts=BindgenOptions {
        convert_macros: true,
        ..Default::default()
    };
    assert_bind_eq(opts, "headers/defines.h", "
        pub const FLAG_10: ::std::os::raw::c_ushort = 512;
        pub const ERROR: ::std::os::raw::c_char = -1;
    ");
}
