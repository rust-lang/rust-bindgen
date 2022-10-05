// bindgen-flags: --enable-cxx-namespaces

namespace ns {
enum class AB { A, B };
}
using AB = ns::AB;
static const AB kA = AB::A;
