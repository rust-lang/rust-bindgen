// bindgen-flags: --translate-function-macros

// Simple arithmetic
#define ADD(x, y) ((x) + (y))
#define DOUBLE(x) ((x) * 2)
#define FLAG(n) (1 << (n))

// Bitwise NOT (~ → !)
#define COMPLEMENT(x) (~(x))

// Ternary
#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define ABS(x) ((x) >= 0 ? (x) : -(x))

// C casts
#define TO_UNSIGNED(x) ((unsigned int)(x))

// sizeof with generic type parameter
#define SIZEOF_T(T) sizeof(T)

// Composing macros
#define IOC(dir, type, nr, size) (((dir) << 30) | ((type) << 8) | (nr) | ((size) << 16))
#define IO(type, nr) IOC(0, (type), (nr), 0)

// Keyword parameter escaping
#define SHIFT(type, n) ((type) << (n))

// Number literal suffix translation
#define BIG 42UL
#define MASK(n) (0xFFUL << (n))
