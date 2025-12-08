// Test that __BindgenBitfieldUnit gets the union of derives from all structs using it

/** <div rustbindgen derive="PartialEq"></div> */
struct first_bitfield_struct {
    unsigned int a : 1;
    unsigned int b : 2;
    unsigned int c : 3;
};

/** <div rustbindgen derive="PartialEq, Eq"></div> */
struct second_bitfield_struct {
    unsigned int x : 4;
    unsigned int y : 4;
};

// Third struct with no custom derives but should still work
struct third_bitfield_struct {
    unsigned int z : 8;
};
