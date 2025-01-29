// bindgen-flags: --result-error-enum "MyResult" --rust-target 1.79

enum MyResult {
    MyResultOk = 0,
    MyResultErr1,
    MyResultErr2,
};

typedef enum MyResult ResultTypedef;

enum MyResult do_something(void);

ResultTypedef do_something2(void);
