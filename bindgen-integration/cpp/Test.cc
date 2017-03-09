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

}
