// bindgen-flags: -- -Itests/headers/source-order

const int THIS_SHOULD_BE_FIRST = 1;

#include "source-order-2.h"

extern int THIS_SHOULD_BE_FIFTH;
