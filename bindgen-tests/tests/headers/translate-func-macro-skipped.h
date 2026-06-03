// bindgen-flags: --translate-function-macros

// These should all be skipped (not appear in output).
#define VARIADIC(fmt, ...) fmt
#define HAS_VOID(x) ((void*)(x))
#define EMPTY_BODY()
#define STMT_MACRO(x) do { x; } while(0)
#define CALL_PTR(f, x) (f)(x)

// This should still appear.
#define GOOD(x) ((x) + 1)
