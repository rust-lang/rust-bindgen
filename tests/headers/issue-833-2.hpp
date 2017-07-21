// bindgen-flags: --raw-line "// If the output of this changes, please ensure issue-833-1.hpp changes too"

template<typename T>
class nsTArray {
  static T* sFoo;
};
