// bindgen-flags: --enable-cxx-namespaces --allowlist-type '.*'

namespace outer {
  namespace inner {
    struct Helper {};
  }
  struct Test {
    inner::Helper helper;
  };
}
