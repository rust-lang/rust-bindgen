#include "/home/volker/Sync/git/rust-bindgen/tests/headers/issue-447.hpp"
template <typename T>
auto bindgen_destruct_or_throw(T* t) -> decltype(t->~T()) {
    t->~T();
}
auto bindgen_destruct_or_throw(void*) -> void {
    /*Todo: throw an exception or abort here*/
}
void bindgen_destruct_mozilla_detail_GuardObjectNotifier(struct mozilla::detail::GuardObjectNotifier *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_destruct_mozilla_c(struct mozilla::c *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_destruct_js_D(struct js::D *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_destruct_js_ContextFriendFields(struct js::ContextFriendFields *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_destruct_f(struct f *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_destruct_JSAutoCompartment(struct JSAutoCompartment *ptr) {
    bindgen_destruct_or_throw(ptr);
}
