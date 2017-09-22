// bindgen-flags: --opaque-type 'OpaqueTemplate' --with-derive-hash --with-derive-partialeq --impl-partialeq --with-derive-eq

template<typename T>
class OpaqueTemplate {
    T mData;
    bool mCannotDebug[400];
};

/// This should not end up deriving Debug/Hash because its `mBlah` field cannot derive
/// Debug/Hash because the instantiation's definition cannot derive Debug/Hash.
class ContainsOpaqueTemplate {
    OpaqueTemplate<int> mBlah;
    int mBaz;
};

/// This should not end up deriving Debug/Hash either, for similar reasons, although
/// we're exercising base member edges now.
class InheritsOpaqueTemplate : public OpaqueTemplate<bool> {
    char* wow;
};
