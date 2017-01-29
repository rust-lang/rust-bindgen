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
