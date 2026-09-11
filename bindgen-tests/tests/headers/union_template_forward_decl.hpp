// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq
//
template <typename value_t>
union declare_union; // Primary template declared, but never defined.

template <typename value_t> union declare_union<value_t *> {
  declare_union() {}
  declare_union(value_t *a_value) : value(a_value) {}
  value_t *value;
};

template <typename value_t> union define_union {
  define_union() {}
  define_union(value_t *a_value) : value(a_value) {}
  value_t *value;
  int dummy;
};
