// bindgen-flags: --enable-cxx-namespaces

namespace Halide {
struct Type;
}
typedef enum {} a;
namespace Halide {
struct Type {
  static a b;
};
}
