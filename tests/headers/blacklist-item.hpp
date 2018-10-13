// bindgen-flags: --blacklist-item "SomeFunction" --blacklist-item "SOME_DEFUN" --blacklist-item "someVar" --blacklist-item "ExternFunction" --blacklist-item "foo::NamespacedFunction" --blacklist-item "someClass" --enable-cxx-namespaces

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
