// Test that opaque arrays get the union of derives from all structs using them

/** <div rustbindgen derive="PartialEq"></div> */
struct first_struct {
    int x;
    char padding_to_align[60] __attribute__((aligned(64)));
};

/** <div rustbindgen derive="PartialEq, Eq"></div> */
struct second_struct {
    int y;
    char more_padding[60] __attribute__((aligned(64)));
};

// Third struct with no custom derives but should still work
struct third_struct {
    int z;
    char final_padding[60] __attribute__((aligned(64)));
};
