// bindgen-flags: --translate-function-macros

// Bare comparisons: C returns int (0 or 1), Rust returns bool.
// Must cast to the value type so the result is integer-typed.
#define GT(x, y) ((x) > (y))
#define LT(x, y) ((x) < (y))
#define EQ(x, y) ((x) == (y))
#define NE(x, y) ((x) != (y))
#define GTE(x, y) ((x) >= (y))
#define LTE(x, y) ((x) <= (y))

// Without outer parens.
#define GT_BARE(x, y) (x) > (y)

// Deeply nested parens around comparison.
#define NESTED(x, y) (((((x) > (y)))))

// Comparison result used in arithmetic (bool + bool would fail).
#define CMP_ADD(x, y) ((x) > (y)) + ((x) < (y))
#define CMP_MUL(x, y) (((x) > (y)) * 2)

// Comparison with sizeof.
#define IS_BIG(T) (sizeof(T) > 4)
