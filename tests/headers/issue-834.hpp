// bindgen-flags: --whitelist-type U --generate types

struct T {};
struct U {
  void test(T a);
};
