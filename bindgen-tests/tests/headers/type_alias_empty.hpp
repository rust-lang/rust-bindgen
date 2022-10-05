// bindgen-flags: --allowlist-type bool_constant -- -std=c++11

// NB: The --allowlist-type is done to trigger the traversal of the types on
// codegen in order to trigger #67.

template<typename T, T Val>
struct integral_constant {};

template<bool B>
using bool_constant = integral_constant<bool, B>;
