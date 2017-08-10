// bindgen-flags: --opaque-type 'OpaqueTemplate'  --with-derive-hash --with-derive-partialeq

template<typename T>
class OpaqueTemplate {
    T mData;
    bool mCannotDebug[400];
};

/// This should not end up deriving Debug/Hash/PartialEq because its `mBlah` field cannot derive
/// Debug/Hash/PartialEq because the instantiation's definition cannot derive Debug/Hash/PartialEq.
class ContainsOpaqueTemplate {
    OpaqueTemplate<int> mBlah;
    int mBaz;
};

/// This shold not end up deriving Debug/Hash/PartialEq either, for similar reasons, although
/// we're exercising base member edges now.
class InheritsOpaqueTemplate : public OpaqueTemplate<bool> {
    char* wow;
};
