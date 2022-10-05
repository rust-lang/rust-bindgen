// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq


template <typename T>
class HandleWithDtor {
    T* ptr;
    ~HandleWithDtor() {}
};

typedef HandleWithDtor<int> HandleValue;

class WithoutDtor {
    HandleValue shouldBeWithDtor;
};
