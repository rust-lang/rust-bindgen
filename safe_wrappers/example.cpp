#include <cstdio>

void *__gxx_personality_v0; // I don't know what this is, but we get a link error without it

class Member {
public:
    Member();
    ~Member();
};
class Base {

    Member val;
public:
    Base();
    //~Base();
};

Member::Member() {
    printf("Constructing Member\n");
}
Member::~Member() {
    printf("Destructing Member\n");
}
Base::Base() {
    printf("Constructing Base\n");
}
// Base::~Base() {
//     printf("Destructing Base\n");
// }

class Parent {
public:
	virtual ~Parent();
};
Parent::~Parent() {
    printf("Destructing Parent\n");
}
class Child: Parent {
public:
	Child();
	~Child();
    void meth_void();
    int meth_int();
};
void Child::meth_void() {
    printf("meth_void\n");
}
int Child::meth_int() {
    printf("meth_int\n");
    return 123;
}
Child::Child() {
    printf("Constructing Child\n");
}
Child::~Child() {
    printf("Destructing Child\n");
}
//int main(){}
Base factory(int i) {
    Base ret;
    return ret;
}

// TODO: namespaces currently break the cpp wrappers
// namespace MySpace {
//     class SC {
//         public:
//         ~SC();
//         int member();
//     };
//     SC::~SC(){}
//     int SC::member(){}
// }