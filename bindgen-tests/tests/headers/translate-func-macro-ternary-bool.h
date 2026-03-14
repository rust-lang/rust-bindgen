// bindgen-flags: --translate-function-macros

// `!x` in ternary condition: logical NOT, gets (x == 0) translation.
#define BOOL_NOT(x) (!x ? 1 : 0)

// &&/|| in ternary condition: operands wrapped with != 0.
#define BOOL_AND(x, y) ((x) && (y) ? 1 : 0)
#define BOOL_OR(x, y) ((x) || (y) ? 1 : 0)

// Comparison operators produce bool directly (no wrapper).
#define CMP_GT(x, y) ((x) > (y) ? (x) : (y))

// Comparisons + logical ops: comparison sides are already bool.
#define RANGE(x) ((x) > 0 && (x) < 10 ? 1 : 0)

// Chained logical ops with comparisons — each sub-expression with
// both a comparison and a logical op is integer-typed, not bool.
#define ALL3(x, y, z) ((x) > 0 && (y) > 0 && (z) > 0 ? 1 : 0)
#define ANY3(x, y, z) ((x) > 0 || (y) > 0 || (z) > 0 ? 1 : 0)

// Mixed precedence: || splits first, && stays in right sub-expression.
#define MIXED3(x, y, z) ((x) > 0 || (y) > 0 && (z) > 0 ? 1 : 0)
