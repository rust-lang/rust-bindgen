// bindgen-flags: --translate-function-macros

// M takes a type param T (via sizeof) and a value param x.
// N forwards T and y to M, so N also gets T as a type param.
#define M(T, x) (sizeof(T) + (x))
#define N(T, y) M(T, y)
