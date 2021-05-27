// bindgen-flags: --enable-cxx-namespaces

class C {
public:
    typedef int MyInt;

    MyInt method();
};
// class A {};
// class B {
// public:
//     int meth1(int arg);
//     int meth2(A arg);
//     A meth3(int arg);
//     void meth4(A arg);
// };

// int func1(int arg);
// int func2(A arg);
// A func3(int arg);
// void func4(A arg);