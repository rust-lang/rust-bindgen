// bindgen-flags: --blacklist-function "ExternFunction" --blacklist-function "foo::NamespacedFunction" --blacklist-function "C_ClassMethod" --enable-cxx-namespaces

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
