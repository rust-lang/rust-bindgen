// bindgen-flags: --enable-cxx-namespaces --allowlist-type CapturingContentInfo --opaque-type 'mozilla::Maybe' -- -std=c++14

namespace mozilla {
template <class T> class Maybe { using ValueType = T; };
}
struct CapturingContentInfo {
  mozilla::Maybe<float> a;
};
