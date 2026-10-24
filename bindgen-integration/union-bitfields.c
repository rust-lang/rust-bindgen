#include "union-bitfields.h"

unsigned int union_bits_size(void) { return sizeof(union Union); }
int union_bits_get_b(const union Union *value) { return value->bitfld_b; }
void union_bits_set_c(union Union *value, int bits) { value->bitfld_c = bits; }
unsigned int wide_first_size(void) { return sizeof(union WideFirst); }
unsigned int wide_first_get_wide(const union WideFirst *value) { return value->wide; }
unsigned int wide_first_get_narrow(const union WideFirst *value) { return value->narrow; }
void wide_first_set_wide(union WideFirst *value, unsigned int bits) { value->wide = bits; }
unsigned int wide64_size(void) { return sizeof(union Wide64); }
unsigned long long wide64_get_wide(const union Wide64 *value) { return value->wide; }
void wide64_set_wide(union Wide64 *value, unsigned long long bits) { value->wide = bits; }
union WideFirst wide_first_by_value(union WideFirst value, unsigned int bits) {
    value.wide ^= bits;
    return value;
}
union MixedScalar mixed_by_value(union MixedScalar value, unsigned long long bits) {
    value.wide ^= bits;
    return value;
}
