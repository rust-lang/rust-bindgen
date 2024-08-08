// bindgen-flags: -- -Itests/headers/source-order-nested

const int THIS_SHOULD_BE_FIRST = 1;

#include "source-order-nested-2.h"

extern int THIS_SHOULD_BE_FIFTH;
