// bindgen-flags: --with-derive-hash
//
/// Pointers can derive hash
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
