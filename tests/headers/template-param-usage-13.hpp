// bindgen-flags: -- -std=c++14

template <typename T>
class BaseIgnoresT {
    int x;
};

template <typename U>
class CrtpUsesU : public BaseIgnoresT<CrtpUsesU<U>> {
    U usage;
};
