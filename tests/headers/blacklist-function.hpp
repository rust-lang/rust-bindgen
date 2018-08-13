// bindgen-flags: --blacklist-function "ExternFunction" --blacklist-function "foo::NamespacedFunction" --enable-cxx-namespaces

extern "C" void ExternFunction();

namespace foo {
  void NamespacedFunction();
}

namespace bar {
  void NamespacedFunction();
}
