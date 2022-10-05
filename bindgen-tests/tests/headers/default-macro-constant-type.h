// Test default of --default-macro-constant-type
// Negative values are i32 or i64; others are u32 or u64.

#define N0 0
#define N1 1ULL
#define N2 2ULL

#define N_1 (-1LL)
#define N_2 (-2LL)

#define MAX_U16  0xFFFFULL
#define MAX_I16 (0x8000ULL - 1)

#define MAX_I16_Plus1 (MAX_I16 + 1)
#define MAX_U16_Plus1 (MAX_U16 + 1)

#define MAX_I16_Minus1 (MAX_I16 - 1)
#define MAX_U16_Minus1 (MAX_U16 - 1)

#define MIN_U16 0
#define MIN_I16 (- (1ULL<<15))

#define MIN_U16_Plus1 (MIN_U16 + 1)
#define MIN_I16_Plus1 (MIN_I16 + 1)

#define MIN_U16_Minus1 (MIN_U16 - 1)
#define MIN_I16_Minus1 (MIN_I16 - 1)

#define MAX_U32  0xFFFFFFFFULL
#define MAX_I32 (0x80000000ULL - 1)

#define MAX_I32_Plus1 (MAX_I32 + 1)
#define MAX_U32_Plus1 (MAX_U32 + 1)

#define MAX_I32_Minus1 (MAX_I32 - 1)
#define MAX_U32_Minus1 (MAX_U32 - 1)

#define MIN_U32 0
#define MIN_I32 (- (1ULL<<31))

#define MIN_U32_Plus1 (MIN_U32 + 1)
#define MIN_I32_Plus1 (MIN_I32 + 1)

#define MIN_U32_Minus1 (MIN_U32 - 1)
#define MIN_I32_Minus1 (MIN_I32 - 1)

#define LONG12    123456789012ULL
#define LONG_12 (- 123456789012ULL)

// Function parameter and return types are not affected.
int foo(int, signed, unsigned, char, unsigned char, signed char);
long bar(long, long long);
