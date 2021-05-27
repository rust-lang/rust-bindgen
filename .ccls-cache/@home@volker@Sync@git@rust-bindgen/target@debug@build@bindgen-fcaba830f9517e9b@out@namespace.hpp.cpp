#include "/home/volker/Sync/git/rust-bindgen/tests/headers/namespace.hpp"
template <typename T>
auto bindgen_destruct_or_throw(T* t) -> decltype(t->~T()) {
    t->~T();
}
auto bindgen_destruct_or_throw(void*) -> void {
    /*Todo: throw an exception or abort here*/
}
void bindgen_destruct__bindgen_mod_id_17_A(A *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_wrap_w_heh( w::whatever_int_t *writeback) {
    auto val = w::heh();
    *writeback = val;
}
