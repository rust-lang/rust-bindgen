// bindgen-flags: --blocklist-file ".*/blocklisted/file.*" -- -Itests/headers

#include "blocklisted/file.hpp"
#include "blocklisted/fake-stdint.h"

struct SizedIntegers {
    uint8_t x;
    uint16_t y;
    uint32_t z;
};
