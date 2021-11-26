// bindgen-flags: --blocklist-file ".*/blocklisted/file.*" -- -Itests/headers

// Forward declaration of struct that's defined in a blocklisted file.
struct StructWithBlocklistedDefinition;

#include "blocklisted/file.hpp"
#include "blocklisted/fake-stdint.h"

struct SizedIntegers {
    uint8_t x;
    uint16_t y;
    uint32_t z;
};

// Actual definition of struct that has a forward declaration in a blocklisted file.
struct StructWithBlocklistedFwdDecl {
    uint8_t b;
};
