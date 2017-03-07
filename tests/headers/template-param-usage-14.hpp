// bindgen-flags: -- -std=c++14

template <typename T>
class BaseIgnoresT {
    int x;
};

template <typename U>
class CrtpIgnoresU : public BaseIgnoresT<CrtpIgnoresU<U>> {
    int y;
};
