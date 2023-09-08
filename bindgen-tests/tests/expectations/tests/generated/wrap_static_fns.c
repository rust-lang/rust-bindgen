#include "tests/headers/wrap-static-fns.h"

// Static wrappers

int foo__extern(void) { return foo(); }
int bar__extern(void) { return bar(); }
int takes_ptr__extern(int *arg) { return takes_ptr(arg); }
int takes_fn_ptr__extern(int (*f) (int)) { return takes_fn_ptr(f); }
int takes_fn__extern(int (f) (int)) { return takes_fn(f); }
int takes_alias__extern(func f) { return takes_alias(f); }
int takes_qualified__extern(const int *const *arg) { return takes_qualified(arg); }
enum foo takes_enum__extern(const enum foo f) { return takes_enum(f); }
void nevermore__extern(void) { nevermore(); }
int takes_fn_with_no_args__extern(int (f) (void)) { return takes_fn_with_no_args(f); }
void no_extra_argument__extern(__builtin_va_list va) { no_extra_argument(va); }
int many_va_list__extern(int i, __builtin_va_list va1, __builtin_va_list va2) { return many_va_list(i, va1, va2); }
int wrap_as_variadic_fn1__extern(int i, ...) {
    int ret;
    va_list ap;

    va_start(ap, i);
    ret = wrap_as_variadic_fn1(i, ap);
    va_end(ap);
    return ret;
}
void wrap_as_variadic_fn2__extern(int i, ...) {
    va_list ap;

    va_start(ap, i);
    wrap_as_variadic_fn2(i, ap);
    va_end(ap);
}
