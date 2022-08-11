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
