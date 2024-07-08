// bindgen-flags: --no-derive-debug --no-derive-copy --no-derive-default --default-enum-style rust --no-layout-tests

/** <div rustbindgen attribute="#[derive(Debug)]"></div> */
struct my_type;

/** <div rustbindgen attribute="#[derive(Clone)]"></div> */
struct my_type;

struct my_type {
    int a;
};

/**
 * <div rustbindgen attribute="#[derive(Debug)]"></div>
 * <div rustbindgen attribute="#[derive(Clone)]"></div>
 */
struct my_type2;

struct my_type2 {
    unsigned a;
};

/**
 * <div rustbindgen attribute="#[derive(Debug)]" attribute="#[derive(Clone)]"></div>
 */
struct my_type3 {
    unsigned long a;
};
