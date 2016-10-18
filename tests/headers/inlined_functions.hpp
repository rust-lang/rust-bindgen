// bindgen-flags: --keep-inline-functions

inline bool foo() {
  return true;
}

class C {
public:
  static void foo() { }
  static bool bar() { return false; }
};
