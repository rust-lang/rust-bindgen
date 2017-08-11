// bindgen-flags: --with-derive-hash --with-derive-partialeq
//
/// Pointers can derive hash/PartialEq
struct ConstPtrMutObj {
    int* const bar;
};

struct MutPtrMutObj {
    int* bar;
};

struct MutPtrConstObj {
    const int* bar;
};

struct ConstPtrConstObj {
    const int* const bar;
};
