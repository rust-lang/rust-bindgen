class A {
public:
    int member_a;
    class B {
        int member_b;
    };
};

A::B var;

class D {
    A::B member;
};
