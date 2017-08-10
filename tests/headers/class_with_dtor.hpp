// bindgen-flags: --with-derive-hash --with-derive-partialeq


template <typename T>
class HandleWithDtor {
    T* ptr;
    ~HandleWithDtor() {}
};

typedef HandleWithDtor<int> HandleValue;

class WithoutDtor {
    HandleValue shouldBeWithDtor;
};
