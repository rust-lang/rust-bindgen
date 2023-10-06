#include <stdint.h>
#include <stdbool.h>
#include "tests/headers/function_macros.h"

// Macro function wrappers

void SIMPLE__extern(void) { SIMPLE; }
void INDIRECT_SIMPLE__extern(void) { INDIRECT_SIMPLE; }
float COMPLEX__extern(uint32_t x) { return COMPLEX(x); }
float INDIRECT_COMPLEX__extern(uint32_t x) { return INDIRECT_COMPLEX(x); }
float CONDITIONAL_COMPLEX__extern(bool condition, uint32_t x) { return CONDITIONAL_COMPLEX(condition, x); }
