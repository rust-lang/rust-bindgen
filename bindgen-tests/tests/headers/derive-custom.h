// bindgen-flags: --no-derive-debug --no-derive-copy --default-enum-style rust --no-layout-tests

/** <div rustbindgen derive="Debug"></div> */
struct my_type;

/** <div rustbindgen derive="Clone"></div> */
struct my_type;

struct my_type {
    int a;
};

/**
 * <div rustbindgen derive="Debug"></div>
 * <div rustbindgen derive="Clone"></div>
 */
struct my_type2;

struct my_type2 {
    unsigned a;
};

/**
 * <div rustbindgen derive="Debug" derive="Clone"></div>
 */
struct my_type3 {
    unsigned long a;
};
