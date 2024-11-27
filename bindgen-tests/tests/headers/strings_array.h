// bindgen-flags: \-\-rust-target=1.33

const char* MY_STRING_UTF8 = "Hello, world!";
const char* MY_STRING_INTERIOR_NULL = "Hello,\0World!";
const char* MY_STRING_NON_UTF8 = "ABCDE\xFF";
