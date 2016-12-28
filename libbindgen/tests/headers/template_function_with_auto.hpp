// bindgen-flags: -- -std=c++14

template<typename T> auto Test1() {
  return T(1);
}

auto Test2() {
  return Test1<unsigned int>();
}
