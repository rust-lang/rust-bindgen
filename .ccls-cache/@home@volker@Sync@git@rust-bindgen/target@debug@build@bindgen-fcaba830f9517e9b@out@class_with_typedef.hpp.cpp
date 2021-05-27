#include "/home/volker/Sync/git/rust-bindgen/tests/headers/class_with_typedef.hpp"
template <typename T>
auto bindgen_destruct_or_throw(T* t) -> decltype(t->~T()) {
    t->~T();
}
auto bindgen_destruct_or_throw(void*) -> void {
    /*Todo: throw an exception or abort here*/
}
void bindgen_destruct_C(struct C *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_wrap_C_method(C *this_ptr, MyInt *arg_1) {
    return this_ptr->method(*arg_1);
}
void bindgen_wrap_C_anotherMethod(C *this_ptr, AnotherInt *arg_1) {
    return this_ptr->anotherMethod(*arg_1);
}
void bindgen_destruct_D(struct D *ptr) {
    bindgen_destruct_or_throw(ptr);
}
