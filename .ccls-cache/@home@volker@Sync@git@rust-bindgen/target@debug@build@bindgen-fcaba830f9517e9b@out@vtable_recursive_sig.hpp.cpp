#include "/home/volker/Sync/git/rust-bindgen/tests/headers/vtable_recursive_sig.hpp"
template <typename T>
auto bindgen_destruct_or_throw(T* t) -> decltype(t->~T()) {
    t->~T();
}
auto bindgen_destruct_or_throw(void*) {
    /*Todo: throw an exception or abort here*/
}
void bindgen_destruct_Base(Base *ptr){
    bindgen_destruct_or_throw(ptr);
}
void bindgen_wrap_Base_AsDerived(Base *this_ptr,  UNKOWNTYPE *writeback) {
    auto val = this_ptr->AsDerived();
    *writeback = val;
}
void bindgen_destruct_Derived(Derived *ptr){
    bindgen_destruct_or_throw(ptr);
}
