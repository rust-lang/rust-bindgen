// bindgen-flags: --enable-cxx-namespaces --whitelist-type CapturingContentInfo --opaque-type 'mozilla::Maybe' -- -- -std=c++14

namespace mozilla {
template <class T> class Maybe { using ValueType = T; };
}
struct CapturingContentInfo {
  mozilla::Maybe<float> a;
};
