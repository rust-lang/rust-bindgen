// bindgen-flags: --generate-inline-functions

inline static int foo(const int x, const int y) { return x + y; }
static int bar(const int x, const int y) { return x - y; }
inline static void nop() { return; }
