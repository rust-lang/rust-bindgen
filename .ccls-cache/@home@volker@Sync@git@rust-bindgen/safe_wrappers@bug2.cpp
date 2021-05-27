#include <bits/c++config.h>
namespace std _GLIBCXX_VISIBILITY(default) {
 _GLIBCXX_BEGIN_NAMESPACE_VERSION   template<typename _Tp, _Tp __v>     struct integral_constant     {       static constexpr _Tp                  value = __v;       typedef _Tp                           value_type;       typedef integral_constant<_Tp, __v>   type;       constexpr operator value_type() const noexcept { return value; }     };
   typedef integral_constant<bool, false>    false_type;
   template<bool __v>     using __bool_constant = integral_constant<bool, __v>;
   template<typename>     struct is_reference;
   template<typename>     struct is_function;
   template<typename _Tp>     struct is_function<_Tp&&>     : public false_type { };
#define _GLIBCXX_HAS_NESTED_TYPE(_NTYPE)				\
    { };
_GLIBCXX_END_NAMESPACE_VERSION }
