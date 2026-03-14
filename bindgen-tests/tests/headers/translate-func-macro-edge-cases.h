// bindgen-flags: --translate-function-macros

// (type)-literal and (type)-(expr): valid C casts with unary prefix.
// Previously panicked on make_ident("-").
#define NEG() ((int)-1)
#define NEGX(x) ((int)-(x))

// Float ternary with integer literal comparison: the `0` must be
// cast to f64 when value type is float.
#define POS(x) ((x) > 0 ? 1.0f : 0.0f)

// Deterministic type inference: when tied, result should be
// consistent (not depend on HashMap iteration order).
// Both A (u32) and B (i32) are referenced once each.
#define A 1U
#define B -1
#define MIX(x) ((x) + A + B)

// Unsigned suffix inference:
// U (small value) → u32
#define ALL_BITS() (~0U)
#define SMALL_U(x) ((x) + 1U)
// U (value > u32::MAX) → promoted to u64
#define BIG_U() 0x100000000U
#define BIG_OCT_U() 040000000000U
// UL/LU/ULL/LLU/ui64/UI64 → all u64 regardless of order/case
#define HUGE(x) ((x) + 0xFFFFFFFFFFFFFFFFULL)
#define ALL_BITS_UL() (~0UL)
#define ALL_BITS_LU() (~0LU)
#define ALL_BITS_LLU() (~0LLU)
#define ALL_BITS_UI64() (~0ui64)
// ULL with competing signed constants → still u64
#define HUGE_MIXED(x) ((x) + A + B + 0xFFFFFFFFFFFFFFFFULL)

// Octal literals: C 0644 must become Rust 0o644 (not decimal 644).
#define PERMS() 0644
#define OCTAL(x) ((x) + 077)

// non_const_vars bypass: !x and (type)x must also reject extern vars.
extern int ext_var;
#define NOT_EXT() (!ext_var)
#define CAST_EXT() ((int)ext_var)

// These should still work normally.
#define GOOD(x) ((x) + 1)
