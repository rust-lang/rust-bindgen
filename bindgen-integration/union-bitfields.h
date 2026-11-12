#include "../bindgen-tests/tests/headers/union-bitfields-layout.h"

unsigned int union_bits_size(void);
int union_bits_get_b(const union Union *value);
void union_bits_set_c(union Union *value, int bits);
unsigned int wide_first_size(void);
unsigned int wide_first_get_wide(const union WideFirst *value);
unsigned int wide_first_get_narrow(const union WideFirst *value);
void wide_first_set_wide(union WideFirst *value, unsigned int bits);
unsigned int wide64_size(void);
unsigned long long wide64_get_wide(const union Wide64 *value);
void wide64_set_wide(union Wide64 *value, unsigned long long bits);
union WideFirst wide_first_by_value(union WideFirst value, unsigned int bits);
union MixedScalar mixed_by_value(union MixedScalar value, unsigned long long bits);
