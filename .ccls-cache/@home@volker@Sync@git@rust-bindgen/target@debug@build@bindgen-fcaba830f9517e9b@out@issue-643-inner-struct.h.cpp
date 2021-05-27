#include "/home/volker/Sync/git/rust-bindgen/tests/headers/issue-643-inner-struct.h"
template <typename T>
auto bindgen_destruct_or_throw(T* t) -> decltype(t->~T()) {
    t->~T();
}
auto bindgen_destruct_or_throw(void*) -> void {
    /*Todo: throw an exception or abort here*/
}
void bindgen_destruct_rte_ring_prod(rte_ring::prod *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_destruct_rte_ring_cons(rte_ring::cons *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_destruct_rte_ring(rte_ring *ptr) {
    bindgen_destruct_or_throw(ptr);
}
