// bindgen-flags: -- -std=c23

// Binary integer literals (C23) - 0b10 is 2 in decimal

#define DEFINE_BIN_LITERAL 0b10
#define DEFINE_NEG_BIN_LITERAL -0b10
const int CONST_INT_BIN_LITERAL = 0b10;
const int CONST_INT_NEG_BIN_LITERAL = -0b10;

// Octal integer literals - 010 is 8 in decimal

#define DEFINE_OCT_LITERAL 010
#define DEFINE_NEG_OCT_LITERAL -010
const int CONST_INT_OCT_LITERAL = 010;
const int COSNT_INT_NEG_OCT_LITERAL = -010;

// Hexadecimal integer literals - 0x10 is 16 in decimal

#define DEFINE_HEX_LITERAL 0x10
#define DEFINE_NEG_HEX_LITERAL -0x10
const int CONST_INT_HEX_LITERAL = 0x10;
const int CONST_INT_NEG_HEX_LITERAL = -0x10;

// Default decimal integer literals - 10 is 10 in decimal

#define DEFINE_DEC_LITERAL 10
#define DEFINE_NEG_DEC_LITERAL -10
const int CONST_INT_DEC_LITERAL = 10;
const int CONST_INT_NEG_DEC_LITERAL = -10;

// Enums with binary, octal, and hexadecimal integer literals

enum MultiRadixLiteral {
    ENUM_BIN_LITERAL = 0b10,
    ENUM_NEG_BIN_LITERAL = -0b10,
    ENUM_OCT_LITERAL = 010,
    ENUM_NEG_OCT_LITERAL = -010,
    ENUM_HEX_LITERAL = 0x10,
    ENUM_NEG_HEX_LITERAL = -0x10,
    ENUM_DEC_LITERAL = 10,
    ENUM_NEG_DEC_LITERAL = -10,
};

// Edge cases: minimum i64s

const long long MIN_I64_BIN = -0b1000000000000000000000000000000000000000000000000000000000000000;
const long long MIN_I64_OCT = -01000000000000000000000;
const long long MIN_I64_DEC = -9223372036854775808;
const long long MIN_I64_HEX = -0x8000000000000000;

// Big B or big X

const int BIG_B_BIN = 0B1;
const int BIG_X_HEX = 0XF;
