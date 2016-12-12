#pragma once

class Test final {
  int m_int;
  double m_double;
public:
  static const char* name();
  Test(int foo);
  Test(double foo);
};
