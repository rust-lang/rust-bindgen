// bindgen-flags: --opaque-type 'OpaqueTemplate'

// This is like `opaque-template-inst-member.hpp` except exercising the cases
// where we are OK to derive Debug.

template<typename T>
class OpaqueTemplate {
    T mData;
};

// Should derive Debug.
class ContainsOpaqueTemplate {
    OpaqueTemplate<int> mBlah;
    int mBaz;
};

// Should also derive Debug.
class InheritsOpaqueTemplate : public OpaqueTemplate<bool> {
    char* wow;
};
