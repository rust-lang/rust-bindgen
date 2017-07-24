// bindgen-flags: --opaque-type 'OpaqueTemplate'

template<typename T>
class OpaqueTemplate {
    T mData;
    bool mCannotDebug[400];
};

// This should not end up deriving Debug because its `mBlah` field cannot derive
// Debug because the instantiation's definition cannot derive Debug.
class ContainsOpaqueTemplate {
    OpaqueTemplate<int> mBlah;
    int mBaz;
};

// This shold not end up deriving Debug either, for similar reasons, although
// we're exercising base member edges now.
class InheritsOpaqueTemplate : public OpaqueTemplate<bool> {
    char* wow;
};
