// bindgen-flags: -- -std=c++17

typedef int Function(int);
using FunctionAlias = Function;

struct First {
    Function method;
    static Function static_method;
    int value;
};

struct Second {
    FunctionAlias method;
    int value;
};

struct Virtual {
    virtual Function method;
};

Function free_function;
