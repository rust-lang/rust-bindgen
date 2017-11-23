#pragma once

#define TESTMACRO

class Test {
  int m_int;
  double m_double;
public:
  static const char* name();
  Test(int foo);
  Test(double foo);

  static const int COUNTDOWN[];
  static const int* COUNTDOWN_PTR;
  static const int* countdown();
};

namespace testing {

typedef Test TypeAlias;

} // namespace testing

typedef testing::TypeAlias TypeAlias;

namespace bitfields {

struct First {
    unsigned char three_bits_byte_one : 3;
    // This starts a new byte, leaving 5 bits unused.
    unsigned char :0;

    unsigned char six_bits_byte_two : 6;
    unsigned char two_bits_byte_two : 2;

    /// Returns true if the bitfields match the arguments, false otherwise.
    bool assert(unsigned char first,
                unsigned char second,
                unsigned char third);
};

struct Second {
    int thirty_one_bits : 31;
    bool one_bit : 1;

    /// Returns true if the bitfields match the arguments, false otherwise.
    bool assert(int first,
                bool second);
};

enum ItemKind {
    ITEM_KIND_UNO = 0,
    ITEM_KIND_DOS,
    ITEM_KIND_TRES,
};

struct Third {
    int flags : 28;
    bool is_whatever : 1;
    ItemKind kind : 3;

    /// Returns true if the bitfields match the arguments, false otherwise.
    bool assert(int first, bool second, ItemKind third);
};

enum MyEnum {
    ONE = 0,
    TWO,
    THREE,
    FOUR,
};

struct Fourth {
    MyEnum tag: 2;
    unsigned long ptr: 48;

    /// Returns true if the bitfields match the arguments, false otherwise.
    bool assert(MyEnum tag, unsigned long ptr);
};

struct Date2 {
    unsigned short nWeekDay  : 3;    // 0..7   (3 bits)
    unsigned short nMonthDay : 6;    // 0..31  (6 bits)
    unsigned short nMonth    : 5;    // 0..12  (5 bits)
    unsigned short nYear     : 8;    // 0..100 (8 bits)
    unsigned char byte : 8;

    bool assert(unsigned short nWeekDay,
                unsigned short nMonthDay,
                unsigned short nMonth,
                unsigned short nYear,
                unsigned short byte);
};


struct Fifth {
    unsigned short nWeekDay  : 3;    // 0..7   (3 bits)
    unsigned short nMonthDay : 6;    // 0..31  (6 bits)
    unsigned short nMonth    : 5;    // 0..12  (5 bits)
    unsigned short nYear     : 8;    // 0..100 (8 bits)
    unsigned char byte;

    /// Returns true if the bitfields match the arguments, false otherwise.
    bool assert(unsigned short nWeekDay,
                unsigned short nMonthDay,
                unsigned short nMonth,
                unsigned short nYear,
                unsigned char byte);
};

struct Sixth {
    unsigned char byte;
    unsigned char nWeekDay  : 3;
    unsigned char nMonth    : 5;
    unsigned char nMonthDay : 6;

    /// Returns true if the bitfields match the arguments, false otherwise.
    bool assert(unsigned char byte,
                unsigned char nWeekDay,
                unsigned char nMonth,
                unsigned char nMonthDay);
};

struct Seventh {
    bool first_one_bit : 1;
    unsigned int second_thirty_bits : 30;
    unsigned short third_two_bits : 2;
    unsigned int fourth_thirty_bits : 30;
    unsigned short fifth_two_bits : 2;
    bool sixth_one_bit : 1;
    unsigned int seventh_thirty_bits : 30;

    /// Returns true if the bitfields match the arguments, false otherwise.
    bool assert(bool first,
                int second,
                unsigned short third,
                unsigned int fourth,
                unsigned short fifth,
                bool sixth,
                int seventh);
};

} // namespace bitfields

struct AutoRestoreBool {
  bool* m_ptr;
  bool m_value;

  AutoRestoreBool(bool*);
  ~AutoRestoreBool();
};
