template<typename T>
struct Thing {
    struct Inner {
        T *ptr;
    };

    struct AnotherInner : Inner {
    };
};
