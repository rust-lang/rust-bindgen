// bindgen-flags: --enable-cxx-namespaces --whitelist-type nsCSSValue --opaque-type 'nsRefPtrHashtable' -- -- -std=c++14

template <class PtrType> class nsRefPtrHashtable {
  typedef PtrType *UserDataType;
};
struct a {
  nsRefPtrHashtable<int> b;
};
class nsCSSValue {
  a c;
};
