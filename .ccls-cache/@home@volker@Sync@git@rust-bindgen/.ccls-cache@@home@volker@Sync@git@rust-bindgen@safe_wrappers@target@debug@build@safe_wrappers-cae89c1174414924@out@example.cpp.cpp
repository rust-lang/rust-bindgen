#include "/home/volker/Sync/git/rust-bindgen/safe_wrappers/example.cpp"
template <typename T>
auto bindgen_destruct_or_throw(T* t) -> decltype(t->~T()) {
    t->~T();
}
auto bindgen_destruct_or_throw(void*) -> void {
    /*Todo: throw an exception or abort here*/
}
void bindgen_destruct_A(struct A *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_destruct_B(struct B *ptr) {
    bindgen_destruct_or_throw(ptr);
}
