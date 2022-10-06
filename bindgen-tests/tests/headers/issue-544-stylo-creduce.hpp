// bindgen-flags: -- -std=c++14

template <typename> class a;
template <typename b, typename... c> class a<b(c...)> { a(const a &); };
template <typename b, typename... c> a<b(c...)>::a(const a &) {}
