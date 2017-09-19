// bindgen-flags: --with-derive-hash --with-derive-partialord --with-derive-ord --with-derive-partialeq --with-derive-eq --whitelist-type 'Whitelisted.*' --blacklist-type Blacklisted --raw-line "#[repr(C)] #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)] pub struct Blacklisted<T> {t: T, pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>> }"
//
template <class T>
struct Blacklisted {
    T t;
};

/// This would derive(Hash, Eq, PartialEq) if it didn't contain a blacklisted type,
/// causing us to conservatively avoid deriving hash/Eq/PartialEq for it.
struct WhitelistedOne {
    Blacklisted<int> a;
};

/// This can't derive(Hash/Eq) even if it didn't contain a blacklisted type.
struct WhitelistedTwo {
    Blacklisted<float> b;
};
