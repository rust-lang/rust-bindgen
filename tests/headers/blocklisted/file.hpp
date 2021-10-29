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


struct StructWithBlocklistedFwdDecl;

struct StructWithBlocklistedDefinition {
    StructWithBlocklistedFwdDecl* other;
};
