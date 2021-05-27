#include "/home/volker/Sync/git/rust-bindgen/safe_wrappers/example.cpp"
template <typename T>
auto bindgen_destruct_or_throw(T* t) -> decltype(t->~T()) {{
    t->~T();
}}
auto bindgen_destruct_or_throw(void*) -> void {{}}
void bindgen_destruct_MySpace_c(struct MySpace::c *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_wrap_MySpace_c_factory(MySpace::c *this_ptr,  MySpace::c::priv *writeback) {
    auto val = this_ptr->factory();
    *writeback = val;
}
