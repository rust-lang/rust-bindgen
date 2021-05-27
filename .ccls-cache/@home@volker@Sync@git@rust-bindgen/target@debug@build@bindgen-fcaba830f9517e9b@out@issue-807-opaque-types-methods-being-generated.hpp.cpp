#include "/home/volker/Sync/git/rust-bindgen/tests/headers/issue-807-opaque-types-methods-being-generated.hpp"
template <typename T>
auto bindgen_destruct_or_throw(T* t) -> decltype(t->~T()) {
    t->~T();
}
auto bindgen_destruct_or_throw(void*) -> void {
    /*Todo: throw an exception or abort here*/
}
void bindgen_destruct_Pupper(struct Pupper *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_destruct_Doggo(struct Doggo *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_destruct_SuchWow(struct SuchWow *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_destruct_NoBindingsShouldBeGeneratedForMe1(struct NoBindingsShouldBeGeneratedForMe1 *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_destruct_NoBindingsShouldBeGeneratedForMe2(struct NoBindingsShouldBeGeneratedForMe2 *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_destruct_Opaque(struct Opaque *ptr) {
    bindgen_destruct_or_throw(ptr);
}
void bindgen_wrap_Opaque_eleven_out_of_ten(Opaque *this_ptr,  SuchWow *writeback) {
    auto val = this_ptr->eleven_out_of_ten();
    *writeback = val;
}
void bindgen_destruct_Whitelisted(struct Whitelisted *ptr) {
    bindgen_destruct_or_throw(ptr);
}
