// bindgen-flags: --use-opaque-newtype-wrapper --raw-line '#[repr(transparent)] pub struct __bindgen_marker_Opaque<T: ?Sized>(T);' -- -x c++ -std=c++14

class Bar {};

template <int N>
class Foo {
public:
    int a[N];
};

// Non-type template parameters are one of the cases where bindgen is
// forced to generate an opaque type. Ensure we spot that and annotate
// it.
void take_foo(Foo<3> foo);
