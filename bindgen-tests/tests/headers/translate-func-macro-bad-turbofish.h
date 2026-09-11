// bindgen-flags: --translate-function-macros

// ID takes a value param. WRAP passes sizeof type param T to ID.
// WRAP should be skipped (T is a type but ID expects a value).
#define ID(x) (x)
#define WRAP(T) (ID(T) + sizeof(T))
