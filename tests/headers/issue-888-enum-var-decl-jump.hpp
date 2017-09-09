// bindgen-flags: --enable-cxx-namespaces --rustified-enum .*

namespace Halide {
struct Type;
}
typedef enum {} a;
namespace Halide {
struct Type {
  static a b;
};
}
