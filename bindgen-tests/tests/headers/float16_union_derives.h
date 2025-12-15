// bindgen-flags: --with-derive-partialeq
// Test that __BindgenFloat16 gets the union of derives from all structs using it

struct first_float16_struct {
    __fp16 a;
};

struct second_float16_struct {
    __fp16 b;
};

// Third struct with no custom derives but should still work
struct third_float16_struct {
    __fp16 c;
};
