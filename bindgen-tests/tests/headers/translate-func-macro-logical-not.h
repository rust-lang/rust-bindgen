// bindgen-flags: --translate-function-macros

// C logical NOT: !0 = 1, !nonzero = 0.
// Translated to ((operand) as i64 == 0) as vt.

#define NOT(x) (!(x))
#define TOBOOL(x) (!!(x))

// !call(x) — the call must be fully consumed.
#define ID(x) (x)
#define NOT_ID(x) !ID(x)

// !literal with C suffix — suffix must be stripped.
#define NOT_ZERO() !0U

// !sizeof(T) — sizeof must be handled before the call-identifier path.
#define NOT_SIZEOF(T) (!sizeof(T))

// Standalone && / || chains (not in ternary).
#define AND(x, y) ((x) && (y))
#define OR(x, y) ((x) || (y))
#define CHAIN3(a, b, c) ((a) && (b) && (c))
#define MIXED_CHAIN(a, b, c) ((a) || (b) && (c))
