// bindgen-flags: --blocklist-function "ExternFunction" --blocklist-function "foo::NamespacedFunction" --blocklist-function "C_ClassMethod" --enable-cxx-namespaces

extern "C" void ExternFunction();

namespace foo {
  void NamespacedFunction();
}

namespace bar {
  void NamespacedFunction();
}

class C {
public:
  void ClassMethod();
};
