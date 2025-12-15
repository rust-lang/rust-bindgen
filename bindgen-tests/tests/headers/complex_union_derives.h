// bindgen-flags: --with-derive-partialeq
// Test that __BindgenComplex gets the union of derives from all structs using it

struct first_complex_struct {
    double _Complex a;
};

struct second_complex_struct {
    float _Complex b;
};

// Third struct with no custom derives but should still work
struct third_complex_struct {
    double _Complex c;
};
