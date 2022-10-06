// bindgen-flags: --allowlist-type AllowlistMe

template<typename T>
class AllowlistMe {
  class Inner {
    T bar;
  };

  int foo;
  Inner bar;
};

struct DontAllowlistMe {
  void* foo;
  double _Complex noComplexGenerated;
};
