// bindgen-flags: --respect-cxx-access-specs
class PubPriv {
  public:
    int x;
  private:
    int y;
};

class PrivateBitFields {
  unsigned int a : 4;
  unsigned int b : 4;
};
class PublicBitFields {
 public:
  unsigned int a : 4;
  unsigned int b : 4;
};
class MixedBitFields {
  unsigned int a : 4;
 public:
  unsigned int d : 4;
};

class Base {
 public:
  int member;
};

class InheritsPrivately : Base {};
class InheritsPublically : public Base {};

class WithAnonStruct {
  struct {
     int a;
  };
 public:
  struct {
    int b;
  };
};

class WithAnonUnion {
  union {};
};