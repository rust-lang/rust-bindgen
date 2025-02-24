// bindgen-flags: --use-reference-newtype-wrapper --raw-line '#[repr(transparent)] pub struct __bindgen_marker_Reference<T: ?Sized>(*const T); #[repr(transparent)] pub struct __bindgen_marker_RValueReference<T: ?Sized>(*const T);' -- -x c++ -std=c++14

const int& receive_reference(const int& input) {
    return input;
}
int& receive_mut_reference(int& input) {
    return input;
}
void receive_rvalue_reference(int&& input) {
}
