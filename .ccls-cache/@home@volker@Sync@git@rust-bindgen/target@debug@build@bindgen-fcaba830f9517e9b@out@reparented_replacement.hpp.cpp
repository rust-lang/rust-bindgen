#include "/home/volker/Sync/git/rust-bindgen/tests/headers/reparented_replacement.hpp"
template <typename T>
auto bindgen_destruct_or_throw(T* t) -> decltype(t->~T()) {
    t->~T();
}
auto bindgen_destruct_or_throw(void*) -> void {
    /*Todo: throw an exception or abort here*/
}
void bindgen_destruct_foo_Bar(struct foo::Bar_Replacement *ptr) {
    bindgen_destruct_or_throw(ptr);
}
