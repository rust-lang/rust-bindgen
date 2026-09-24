// bindgen-flags: --rustified-repr-c-enum ".*" -- -std=c++11

typedef enum {
    SMALL_VALUE = 0x1,
} small_value_t;

static_assert(sizeof(small_value_t) == 4, "");

typedef enum {
    MEDIUM_VALUE = 0x100,
} medium_value_t;

static_assert(sizeof(medium_value_t) == 4, "");

typedef enum {
    LARGE_VALUE = 0x1000000,
} large_value_t;

static_assert(sizeof(large_value_t) == 4, "");

typedef enum {
    UNSIGNED_VALUE = 0x80000000,
} unsigned_value_t;

static_assert(sizeof(unsigned_value_t) == 4, "");

typedef enum {
    NEGATIVE_VALUE = -1,
    POSITIVE_VALUE = 1,
} mixed_sign_t;

static_assert(sizeof(mixed_sign_t) == 4, "");

typedef enum {
    WIDE_NEGATIVE_VALUE = -1,
    WIDE_UNSIGNED_VALUE = 0x80000000,
} wide_mixed_sign_t;

static_assert(sizeof(wide_mixed_sign_t) == 8, "");
