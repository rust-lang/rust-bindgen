// bindgen-flags: --blocklist-item "SomeFunction" --blocklist-item "SOME_DEFUN" --blocklist-item "someVar" --blocklist-item "ExternFunction" --blocklist-item "foo::NamespacedFunction" --blocklist-item "someClass.*" --enable-cxx-namespaces

void SomeFunction();
extern int someVar;
#define SOME_DEFUN 123

class someClass {
  void somePrivateMethod();
public:
  void somePublicMethod(int foo);
};

extern "C" void ExternFunction();

namespace foo {
  void NamespacedFunction();
}

namespace bar {
  void NamespacedFunction();
}
