// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq
//
template<typename T>
struct NastyStruct {
  bool mIsSome;
  union {
    void* mFoo;
    unsigned long mDummy;
  } mStorage;

  union {
    short wat;
    int* wut;
  };
};

template<typename T>
union Whatever {
  void* mTPtr;
  int mInt;
};
