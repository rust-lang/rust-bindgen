// bindgen-flags: --rustified-enum .* -- -std=c++11 -fshort-enums

typedef enum {
    SOME_VALUE = 0x1,
} one_byte_t;

static_assert(sizeof(one_byte_t) == 1, "Short enums should work");

typedef enum {
    SOME_OTHER_VALUE = 0x100,
} two_byte_t;

static_assert(sizeof(two_byte_t) == 2, "");

typedef enum {
    SOME_BIGGER_VALUE = 0x1000000,
} four_byte_t;

static_assert(sizeof(four_byte_t) == 4, "");
