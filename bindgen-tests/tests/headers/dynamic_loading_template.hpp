// bindgen-flags: --dynamic-loading TestLib

template<typename T>
T foo(T x);

template<>
int foo(int x);

template<>
float foo(float x);
