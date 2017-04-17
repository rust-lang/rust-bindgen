// bindgen-flags: --enable-cxx-namespaces --whitelist-type StaticRefPtr --opaque-type 'JS::Rooted' -- -- -std=c++14

namespace JS {
template <typename T> class Rooted { using ElementType = T; };
}
class c {
  JS::Rooted<int> b;
};
class B {
  c a;
};
template <class> class StaticRefPtr {};
struct {
  StaticRefPtr<B> d;
} e;
