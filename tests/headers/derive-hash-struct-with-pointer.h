// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq
//
/// Pointers can derive hash/PartialEq/Eq
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
