// Dependent library header (like "bar" in the issue)
// bar depends on foo

#include "emit-symbol-list-foo.h"

typedef foo_int_t bar_int_t;

struct bar_struct {
    struct foo_struct foo;
    int z;
};

void bar_function(struct foo_struct* arg, struct bar_struct* arg2);

extern int bar_global_var;
