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
int bindgen_wrap_B_meth1(B *this_ptr, int arg_1) {
    return this_ptr->meth1(arg_1);
}
void bindgen_wrap_B_meth3(B *this_ptr, int arg_1, A *writeback) {
    auto val = this_ptr->meth3(arg_1);
    *writeback = val;
}
int bindgen_wrap_func1(int arg_0) {
    return func1(arg_0);
}
void bindgen_wrap_func3(int arg_0, A *writeback) {
    auto val = func3(arg_0);
    *writeback = val;
}
