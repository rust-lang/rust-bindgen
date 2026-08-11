#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// We should _not_ generate a cstr for this because cstr shouldn't have interior nulls.
pub const FOO: &[u8; 4] = b"a\0b\0";
