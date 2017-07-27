template<typename T>
class nsTArray {
  void* mBuffer;
};

/**
 * This is intended to replace another type, but won't if we treat this include
 * as a system include, because clang doesn't parse comments there.
 *
 * See #848.
 *
 * <div rustbindgen replaces="nsTArray"></div>
 */
template<typename T>
class nsTArray_Simple {
  T* m;
};
