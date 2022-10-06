// bindgen-flags: --opaque-type 'OpaqueTemplate' --with-derive-hash --with-derive-partialeq --with-derive-eq

/// This is like `opaque-template-inst-member.hpp` except exercising the cases
/// where we are OK to derive Debug/Hash/PartialEq.

template<typename T>
class OpaqueTemplate {
    T mData;
};

/// Should derive Debug/Hash/PartialEq.
class ContainsOpaqueTemplate {
    OpaqueTemplate<int> mBlah;
    int mBaz;
};

/// Should also derive Debug/Hash/PartialEq.
class InheritsOpaqueTemplate : public OpaqueTemplate<bool> {
    char* wow;
};
