namespace std
{
  template<typename _Tp, _Tp __v>
    struct integral_constant
    {
      static constexpr _Tp                  value = __v;
    };

  template<typename _Tp, _Tp __v>
    constexpr _Tp integral_constant<_Tp, __v>::value;

  typedef integral_constant<bool, false>    false_type;

  struct __do_is_implicitly_default_constructible_impl
  {
    static false_type __test(...);
  };

}











template<typename typ>
struct integral_constant {
    static typ value = 123;
};

template<typename _Tp>
_Tp integral_constant<_Tp>::value;