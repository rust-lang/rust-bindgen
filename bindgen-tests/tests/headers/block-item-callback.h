// bindgen-flags: --blocklist-item 'list_blocked_.*'
// bindgen-parse-callbacks: block-item

struct blocked_my_struct {
    int a;
};

union blocked_my_union {
    int a;
    double b;
};

enum blocked_my_enum {
    BLOCKED_MY_ENUM_A,
    BLOCKED_MY_ENUM_B,
};

static const int blocked_my_const = 10;

struct non_blocked_my_struct {
    int a;
};

union non_blocked_my_union {
    int a;
    double b;
};

enum non_blocked_my_enum {
    NON_BLOCKED_MY_ENUM_A,
    NON_BLOCKED_MY_ENUM_B,
};

static const int non_blocked_my_const = 10;

struct list_blocked_my_struct {
    int a;
};
