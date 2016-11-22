// bindgen-flags: --enable-cxx-namespaces --whitelist-type outer_inner_Helper

namespace outer {
  namespace inner {
    struct Helper {};
  }
  struct Test {
    inner::Helper helper;
  };
}
