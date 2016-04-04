typedef int AnotherInt;

class C {
public:
    typedef int MyInt;
    MyInt c;
    MyInt* ptr;
    MyInt arr[10];
    AnotherInt d;
    AnotherInt* other_ptr;

    void method(MyInt c) {};
    void anotherMethod(AnotherInt c) {};
};

class D: public C {
public:
    MyInt* ptr;
};
