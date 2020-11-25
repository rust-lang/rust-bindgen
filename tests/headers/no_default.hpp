
/** <div rustbindgen nodefault></div> */
template<typename T>
class DefaultButWait {
    int whatever;
};

template<typename T>
class DefaultButWaitDerived {
    DefaultButWait<T> whatever;
};
