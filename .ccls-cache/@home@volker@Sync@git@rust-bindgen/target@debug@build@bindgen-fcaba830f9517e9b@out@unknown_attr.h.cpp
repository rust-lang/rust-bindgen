#include "/home/volker/Sync/git/rust-bindgen/tests/headers/unknown_attr.h"
template <typename T>
auto bindgen_destruct_or_throw(T* t) -> decltype(t->~T()) {
    t->~T();
}
auto bindgen_destruct_or_throw(void*) -> void {
    /*Todo: throw an exception or abort here*/
}
void bindgen_destruct_max_align_t(max_align_t *ptr) {
    bindgen_destruct_or_throw(ptr);
}
