// bindgen-flags: --whitelist-type WhitelistMe

template<typename T>
class WhitelistMe {
public:
  class Inner {
    T bar;
  };

  int foo;
  Inner bar;
};

struct DontWhitelistMe {
  void* foo;
  double _Complex noComplexGenerated;
};
