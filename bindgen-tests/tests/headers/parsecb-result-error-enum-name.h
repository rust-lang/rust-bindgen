// bindgen-flags: --result-error-enum MyResult --rust-target 1.79
// bindgen-parse-callbacks: result-error-enum-rename

enum MyResult {
    MyResultOk = 0,
    MyResultInvalid,
    MyResultAnotherError,
};

typedef enum MyResult AnotherResult;

enum MyResult some_function(void);

AnotherResult another_function(void);
