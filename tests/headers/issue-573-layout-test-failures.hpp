template <int>
struct UnusedIntTemplateParam {};

template <class>
class Outer {
    static const long SIZE = 1;
    UnusedIntTemplateParam<SIZE> i;
};

class AutoIdVector {
  Outer<int> ar;
};
