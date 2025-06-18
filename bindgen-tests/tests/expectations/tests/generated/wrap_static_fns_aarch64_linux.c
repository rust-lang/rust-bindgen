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
