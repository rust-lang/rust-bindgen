// bindgen-flags: --translate-function-macros

// These produce token sequences that aren't valid Rust expressions.
// The syn validation should reject them.
#define ATTR_MACRO(f) __attribute__((nothrow)) f
#define TWO_IDENTS(a, b) a b
#define ASMNAME(cname) __asm__(cname)

// This should still work.
#define GOOD(x) ((x) + 1)
