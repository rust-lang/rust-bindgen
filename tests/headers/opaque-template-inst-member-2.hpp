// bindgen-flags: --opaque-type 'OpaqueTemplate' --with-derive-hash

/// This is like `opaque-template-inst-member.hpp` except exercising the cases
/// where we are OK to derive Debug/Hash.

template<typename T>
class OpaqueTemplate {
    T mData;
};

/// Should derive Debug/Hash.
class ContainsOpaqueTemplate {
    OpaqueTemplate<int> mBlah;
    int mBaz;
};

/// Should also derive Debug/Hash.
class InheritsOpaqueTemplate : public OpaqueTemplate<bool> {
    char* wow;
};
