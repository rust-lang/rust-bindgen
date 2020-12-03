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