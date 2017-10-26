#include "Test.h"

const int Test::COUNTDOWN[] = { 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0 };
const int* Test::COUNTDOWN_PTR = Test::COUNTDOWN;

const int* Test::countdown() {
  return COUNTDOWN;
}

const char* Test::name() {
  return "Test";
}

Test::Test(int foo)
  : m_int(foo)
  , m_double(0.0)
{}

Test::Test(double foo)
  : m_int(0)
  , m_double(foo)
{}

AutoRestoreBool::AutoRestoreBool(bool* ptr)
  : m_ptr(ptr)
  , m_value(*ptr)
{}

AutoRestoreBool::~AutoRestoreBool() {
  *m_ptr = m_value;
}

namespace bitfields {

bool
First::assert(unsigned char first,
              unsigned char second,
              unsigned char third)
{
    return three_bits_byte_one == first &&
        six_bits_byte_two == second &&
        two_bits_byte_two == third;
}

bool
Second::assert(int first, bool second)
{
    return thirty_one_bits == first && one_bit == second;
}

bool
Third::assert(int first, bool second, ItemKind third)
{
    return flags == first &&
        is_whatever == second &&
        kind == third;
}

bool
Fourth::assert(MyEnum tag, unsigned long ptr)
{
    return this->tag == tag && this->ptr == ptr;
}

bool
Date2::assert(unsigned short nWeekDay,
              unsigned short nMonthDay,
              unsigned short nMonth,
              unsigned short nYear,
              unsigned short byte)
{
    return this->nWeekDay == nWeekDay &&
        this->nMonthDay == nMonthDay &&
        this->nMonth == nMonth &&
        this->nYear == nYear &&
        this->byte == byte;
}

bool
Fifth::assert(unsigned short nWeekDay,
              unsigned short nMonthDay,
              unsigned short nMonth,
              unsigned short nYear,
              unsigned char byte)
{
    return this->nWeekDay == nWeekDay &&
        this->nMonthDay == nMonthDay &&
        this->nMonth == nMonth &&
        this->nYear == nYear &&
        this->byte == byte;
}

bool
Sixth::assert(unsigned char byte,
              unsigned char nWeekDay,
              unsigned char nMonth,
              unsigned char nMonthDay) {
    return this->nWeekDay == nWeekDay &&
        this->nMonthDay == nMonthDay &&
        this->nMonth == nMonth &&
        this->byte == byte;
};

bool
Seventh::assert(bool first,
                int second,
                unsigned short third,
                unsigned int fourth,
                unsigned short fifth,
                bool sixth,
                int seventh) {
  return this->first_one_bit == first &&
      this->second_thirty_bits == second &&
      this->third_two_bits == third &&
      this->fourth_thirty_bits == fourth &&
      this->fifth_two_bits == fifth &&
      this->sixth_one_bit == sixth &&
      this->seventh_thirty_bits == seventh;
};

} // namespace bitfields
