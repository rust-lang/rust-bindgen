// bindgen-flags: --dynamic-loading TestLib --allowlist-var foo --allowlist-var bar

int foo;
int bar;
int baz; // should not be allowed