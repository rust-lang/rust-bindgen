// bindgen-flags: --keep-integer-radices -- -std=c++14
// (C23 is not available in clang 9.0, but C++14 supports the same literals)

// Binary integer literals (C23) - 0b10 is 2 in decimal

#define DEFINE_BIN_LITERAL 0b10
#define DEFINE_NEG_BIN_LITERAL -0b10
const int CONST_INT_BIN_LITERAL = 0b10;
const int CONST_INT_NEG_BIN_LITERAL = -0b10;

// Octal integer literals - 010 is 8 in decimal

#define DEFINE_OCT_LITERAL 010
#define DEFINE_NEG_OCT_LITERAL -010
const int CONST_INT_OCT_LITERAL = 010;
const int CONST_INT_NEG_OCT_LITERAL = -010;

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

// Octal with extra leading zero

const char AGENT = 007;

// C23 and C++14 thousands'/digit separator '

const unsigned long long SEP_BIN = 0b11111111'00000000;
const unsigned long long SEP_OCT = 07777'7777'7777;
const unsigned long long SEP_DEC = 299'792'458;
const unsigned long long SEP_HEX = 0x1111'bbbb'cccc'dddd;

// Multiple declarations

const long BIN_1ST = 0b10101010, OCT_2ND = 0777, DEC_3RD = 1234, HEX_4TH = 0xffff;

// Smaller integer types

const unsigned short USHORT_HEX = 0xFFFF;
const short SHORT_HEX = 0x7FFF;
const unsigned char UCHAR_HEX = 0xFF;
const char CHAR_HEX = 0x7F;
