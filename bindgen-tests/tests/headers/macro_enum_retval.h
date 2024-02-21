// bindgen-flags: --experimental --generate-fn-macros

typedef enum {
    wrong = 0,
    right,
} truth;

#define get_wrong() wrong

