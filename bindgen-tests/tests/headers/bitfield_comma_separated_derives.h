// Test that comma-separated derives in rustbindgen annotations are properly split

/** <div rustbindgen derive="PartialEq,Eq"></div> */
struct first_bitfield_struct {
    unsigned int a : 1;
    unsigned int b : 2;
    unsigned int c : 3;
};

/** <div rustbindgen derive="Clone, Copy"></div> */
struct second_bitfield_struct {
    unsigned int x : 4;
    unsigned int y : 4;
};
