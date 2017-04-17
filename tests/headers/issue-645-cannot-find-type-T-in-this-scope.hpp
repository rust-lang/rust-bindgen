// bindgen-flags: --blacklist-type RefPtr --raw-line "#[derive(Clone, Copy, Debug)] pub struct RefPtr<T>(T);" --whitelist-type "HasRefPtr" -- -std=c++14

template <class> class RefPtr {};

template <typename T>
class HasRefPtr {
  typedef T TypedefOfT;
  RefPtr<TypedefOfT> refptr_member;
};
