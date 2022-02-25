// bindgen-flags: --raw-line '#![cfg(not(target_os="windows"))]' -- -std=c++11 
//
// This test is much like enum_explicit_type, but without --rustified-enum.

#include "enum_explicit_type.hpp"
