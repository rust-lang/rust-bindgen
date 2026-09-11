// bindgen-flags: --translate-function-macros

// Float suffix f/F is stripped; float literals infer f64.
#define ADD_HALF(x) ((x) + 0.5f)
#define ADD_QUARTER(x) ((x) + 0.25F)

// Exponent notation also infers f64.
#define ADD_EXP(x) ((x) + 1e3)
#define ADD_EXPF(x) ((x) + 1e3f)

// Hex literals: f/F are hex digits, NOT suffixes. These stay i64.
#define HEX_MASK(x) ((x) & 0xFF)
#define HEX_COMBO(x) ((x) | 0xABCDEF)

// Float logical NOT: uses == 0.0 instead of as i64.
#define NOT_HALF() !0.5f

// Float ternary with logical NOT.
#define NOT_PARAM(x) (!(x) ? 1.0f : 0.0f)

// Float logical ops: operands wrapped with != 0.0.
#define FLOAT_BOTH() (0.5f && 2.0f ? 1.0f : 0.0f)
#define FLOAT_CHAIN() (0.5f && 2.0f && 3.0f ? 1.0f : 0.0f)
