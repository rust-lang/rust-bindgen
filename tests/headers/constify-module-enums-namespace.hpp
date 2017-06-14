// bindgen-flags: --enable-cxx-namespaces --constified-enum-module foo

namespace ns1 {
  namespace ns2 {
    enum foo {
      THIS,
      SHOULD_BE,
      A_CONSTANT,
    };
  }

  namespace ns3 {
    struct bar {
      enum ns2::foo this_should_work;
    };
  }
}