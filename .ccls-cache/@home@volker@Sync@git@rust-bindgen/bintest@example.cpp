#include "example.hpp"

// #include <cstddef>
// class MyClass {
// public:
//     int val;
//     int other;
//     MyClass operator+(const MyClass& rhs) const;
//     MyClass operator-(const MyClass& rhs) const;
//     MyClass& operator+=(const MyClass& rhs);
//     bool operator==(const MyClass& rhs) const ;
//     int operator[](size_t index) const;
//     MyClass operator-() const;
//     void testfunc();
// };
// MyClass MyClass::operator+(const MyClass& rhs) const{
//     MyClass ret;
//     ret.val = val + rhs.val;
//     return ret;
// }
// MyClass MyClass::operator-(const MyClass& rhs) const{
//     MyClass ret;
//     ret.val = val - rhs.val;
//     return ret;
// }
// MyClass& MyClass::operator+=(const MyClass& rhs) {
//     val += rhs.val;
//     return *this;
// }
// bool MyClass::operator==(const MyClass& rhs) const {
//     return val == rhs.val;
// }
// int MyClass::operator[](size_t index) const {
//     if (index == 0) {
//         return val;
//     } else {
//         return other;
//     }
// }
// MyClass MyClass::operator-() const {
//     MyClass ret;
//     ret.val = -val;
//     return ret;
// }

void *__gxx_personality_v0;


// class SelfRef {
// public:
//     SelfRef *ptr;
//     int i;
//     SelfRef();
//     SelfRef(int i);
//     // ~SelfRef();
//     SelfRef(const SelfRef&) = delete;
//     // SelfRef(const SelfRef &other) = delete;
//     // SelfRef(const SelfRef &other);
//     // SelfRef(SelfRef&& other);
//     // SelfRef& operator=(const SelfRef& other);
//     // SelfRef& operator=(SelfRef&& other);
//     void check() const;
// };

// SelfRef::SelfRef() {
//     ptr = this;
//     i = 123456;
//     printf("constructing %x %x\n", this, ptr);
// }
// SelfRef::SelfRef(int i) {
//     ptr = this;
//     this->i = i;
//     printf("constructing %x\n", this);
// }
// // SelfRef::~SelfRef() {
// //     printf("destructing %x %x\n", this, ptr);
// // }
// // SelfRef::SelfRef(const SelfRef &other) {
// //     printf("copy construct\n");
// // }
// // SelfRef::SelfRef(SelfRef&& other) {
// //     printf("move construct\n");
// // }
// // SelfRef& SelfRef::operator=(const SelfRef& other) {
// //     printf("copy assign\n");
// //     return *this;
// // }
// // SelfRef& SelfRef::operator=(SelfRef&& other) {
// //     printf("move assign\n");
// //     return *this;
// // }
// struct TestStruct {
// public:
//     TestStruct *ptr;
//     int i;
// };
// SelfRef SelfRef_factory() {
//     auto val = SelfRef(4);
//     //auto val = TestStruct{NULL, 4};
//     //val.ptr = &val;
//     return val;
// }
// void SelfRef::check() const {
//     //printf("checking %x %x %i\n", this, ptr, i);
// }

// void testfunc() {
//     auto val1 = SelfRef_factory();
//     auto val2 = SelfRef_factory();
//     //val1.check();
//     //val2.check();
// }


Member::Member() {
    printf("Constructing Member\n");
}
Member::~Member() {
    printf("Destructing Member\n");
}
Base::Base() {
    printf("Constructing Base\n");
}
void deleter(Base *ptr) {
    ptr->~Base();
}
// Base::~Base() {
//     printf("Destructing Base\n");
// }