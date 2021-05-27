#include "/home/volker/Sync/git/rust-bindgen/safe_wrappers/example.cpp"
#include <cstdlib>
template <typename T>
auto bindgen_destruct_or_throw(T* t) -> decltype(t->~T()) {
    t->~T();
}
auto bindgen_destruct_or_throw(void*) -> void {
    std::abort();
}
void bindgen_destruct_A(struct A *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_wrap_A_meth(A *this_ptr, int arg_1, A *arg_2,  A *writeback) {
    auto val = this_ptr->meth(arg_1, *arg_2);
    *writeback = val;
}
