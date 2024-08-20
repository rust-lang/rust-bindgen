// bindgen-flags: -- -Itests/headers -Itests/headers/source-order-recursive

#ifndef A_H
#define A_H

struct foo {};

#include "source-order-recursive-2.h"

struct baz {
    struct bar field;
};

#endif
