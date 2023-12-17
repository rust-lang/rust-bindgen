// bindgen-flags: --rust-target 1.0 -- --target=x86_64-pc-windows-msvc

struct CppObj {
    int x;

    CppObj(int x);
    ~CppObj();
};

struct CppObj2 {
    int x;

    CppObj2(int x);
    virtual ~CppObj2();
};

struct CppObj3 : CppObj2 {
    int x;

    CppObj3(int x);
    virtual ~CppObj3();
};

struct CppObj4 : CppObj2 {
    int x;

    CppObj4(int x);
    ~CppObj4();
};