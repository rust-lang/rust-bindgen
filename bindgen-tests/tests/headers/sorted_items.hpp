// bindgen-flags: --sort-semantically --enable-cxx-namespaces -- --target=x86_64-unknown-linux

int foo();
typedef int number;
int bar(number x);
struct Point
{
    number x;
    number y;
};
struct Angle
{
    number a;
    number b;
};
int baz(struct Point point);
const number NUMBER = 42;

namespace ns {
    int foo();
    typedef int number;
    int bar(number x);
    struct Point
    {
        number x;
        number y;
    };
    struct Angle
    {
        number a;
        number b;
    };
    int baz(struct Point point);
    const number NUMBER = 42;
}
