// bindgen-flags: --generate-cstr

/// We should _not_ generate a cstr for this because cstr shouldn't have interior nulls.
#define FOO "a\0b"
