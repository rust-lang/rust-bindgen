// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq --default-alias-style=new_type -- -std=c++14

template<typename T>
using Wrapped = T;
