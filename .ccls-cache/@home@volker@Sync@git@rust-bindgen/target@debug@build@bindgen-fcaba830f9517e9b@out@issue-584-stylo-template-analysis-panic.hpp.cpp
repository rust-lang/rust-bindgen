#include "/home/volker/Sync/git/rust-bindgen/tests/headers/issue-584-stylo-template-analysis-panic.hpp"
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
void bindgen_destruct_g(struct g *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_destruct_b(struct b *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_wrap_Servo_Element_GetSnapshot(, A *writeback) {
    auto val = Servo_Element_GetSnapshot();
    *writeback = val;
}
