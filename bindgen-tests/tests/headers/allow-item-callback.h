// bindgen-flags: --allowlist-item 'list_allowed_.*'
// bindgen-parse-callbacks: allow-item

struct allowed_my_struct {
    int a;
};

union allowed_my_union {
    int a;
    double b;
};

enum allowed_my_enum {
    ALLOWED_MY_ENUM_A,
    ALLOWED_MY_ENUM_B,
};

static const int allowed_my_const = 10;

struct non_allowed_my_struct {
    int a;
};

union non_allowed_my_union {
    int a;
    double b;
};

enum non_allowed_my_enum {
    NON_ALLOWED_MY_ENUM_A,
    NON_ALLOWED_MY_ENUM_B,
};

static const int non_allowed_my_const = 10;

struct list_allowed_my_struct {
    int a;
};
