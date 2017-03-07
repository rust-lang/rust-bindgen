// bindgen-flags: -- -std=c++14

template <typename T>
class BaseUsesT {
    T* usage;
};

template <typename U>
class CrtpIgnoresU : public BaseUsesT<CrtpIgnoresU<U>> {
    int y;
};
