#include <cstddef>
class MyClass {
public:
    int val;
    int other;
    MyClass operator+(const MyClass& rhs) const;
    MyClass operator-(const MyClass& rhs) const;
    MyClass& operator+=(const MyClass& rhs);
    bool operator==(const MyClass& rhs) const ;
    int operator[](size_t index) const;
    MyClass operator-() const;
};
MyClass MyClass::operator+(const MyClass& rhs) const{
    MyClass ret;
    ret.val = val + rhs.val;
    return ret;
}
MyClass MyClass::operator-(const MyClass& rhs) const{
    MyClass ret;
    ret.val = val - rhs.val;
    return ret;
}
MyClass& MyClass::operator+=(const MyClass& rhs) {
    val += rhs.val;
    return *this;
}
bool MyClass::operator==(const MyClass& rhs) const {
    return val == rhs.val;
}
int MyClass::operator[](size_t index) const {
    if (index == 0) {
        return val;
    } else {
        return other;
    }
}
MyClass MyClass::operator-() const {
    MyClass ret;
    ret.val = -val;
    return ret;
}