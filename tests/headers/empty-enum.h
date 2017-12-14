// bindgen-flags: --rustified-enum '.*Rustified.*' --constified-enum-module '.*Module.*' -- -x c++ --std=c++14

// Constified is default, so no flag for that.

enum EmptyConstified {};
enum EmptyRustified {};
enum EmptyModule {};

enum class EmptyClassRustified : char {};
enum class EmptyClassConstified : char {};
enum class EmptyClassModule : char {};

enum class ForwardClassRustified : char;
enum class ForwardClassConstified : char;
enum class ForwardClassModule : char;
