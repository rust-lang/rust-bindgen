// bindgen-flags: --enable-cxx-namespaces --whitelist-type '.*Helper'

namespace outer {
  namespace inner {
    struct Helper {};
  }
  struct Test {
    inner::Helper helper;
  };
}
