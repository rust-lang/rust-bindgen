// bindgen-flags: --enable-cxx-namespaces --whitelist-type JS::Value --rustified-enum .*

namespace JS {
class Value;
}
typedef enum {} JSWhyMagic;
namespace JS {
class Value {
public:
  void a(JSWhyMagic);
};
}
