#include "/home/volker/Sync/git/rust-bindgen/tests/headers/class_with_inner_struct.hpp"
template <typename T>
auto bindgen_destruct_or_throw(T* t) -> decltype(t->~T()) {
    t->~T();
}
auto bindgen_destruct_or_throw(void*) -> void {
    /*Todo: throw an exception or abort here*/
}
void bindgen_destruct_A_Segment(A::Segment *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_destruct_A(A *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_destruct_B_Segment(B::Segment *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_destruct_B(B *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_destruct_C_Segment(C::Segment *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_destruct_C(C *ptr) {
    bindgen_destruct_or_throw(ptr);
}
