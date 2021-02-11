// bindgen-flags: --enable-cxx-namespaces --allowlist-type outer::inner::Helper

namespace outer {
  namespace inner {
    struct Helper {};
  }
  struct Test {
    inner::Helper helper;
  };
}
