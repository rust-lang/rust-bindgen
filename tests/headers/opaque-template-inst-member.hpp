// bindgen-flags: --opaque-type 'OpaqueTemplate'

template<typename T>
class OpaqueTemplate {
    T mData;
    bool mCannotDebug[40];
};

class ContainsOpaqueTemplate {
    OpaqueTemplate<int> mBlah;
    int mBaz;
};
