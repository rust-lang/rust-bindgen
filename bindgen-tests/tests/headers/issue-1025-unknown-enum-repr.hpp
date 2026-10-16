
template <typename> class a {
  enum {};
};

template <typename> class b {
  enum {
    SOME_VARIANT
  };
};

template <typename> class c {
  enum my_enum {
    MY_ENUM_SOME_VARIANT
  };
};

template <typename> class d {
  enum no_variant_enum {};
};