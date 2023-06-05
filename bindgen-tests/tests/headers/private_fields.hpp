// bindgen-flags: --respect-cxx-access-specs
// bindgen-parse-callbacks: field-visibility-default-public

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

class Override {
 public:
  unsigned int a;
  // override with annotation
  /** <div rustbindgen private></div> */
  unsigned int b;
  // override with callback
  unsigned int private_c;

  unsigned int bf_a : 4;
  // override with annotation
  /** <div rustbindgen private></div> */
  unsigned int bf_b : 4;
  // override with callback
  unsigned int private_bf_c : 4;
};
