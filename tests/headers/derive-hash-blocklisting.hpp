// bindgen-flags: --with-derive-hash --with-derive-partialord --with-derive-ord --with-derive-partialeq --with-derive-eq --allowlist-type 'Allowlisted.*' --blocklist-type Blocklisted --raw-line "#[repr(C)] #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)] pub struct Blocklisted<T> {t: T, pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>> }"
//
template <class T>
struct Blocklisted {
    T t;
};

/// This would derive(Hash, Eq, PartialEq) if it didn't contain a blocklisted type,
/// causing us to conservatively avoid deriving hash/Eq/PartialEq for it.
struct AllowlistedOne {
    Blocklisted<int> a;
};

/// This can't derive(Hash/Eq) even if it didn't contain a blocklisted type.
struct AllowlistedTwo {
    Blocklisted<float> b;
};
