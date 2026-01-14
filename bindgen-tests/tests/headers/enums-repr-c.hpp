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
