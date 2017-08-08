// bindgen-flags: --with-derive-hash --whitelist-type 'Whitelisted.*' --blacklist-type Blacklisted --raw-line "#[repr(C)] #[derive(Debug, Hash, Copy, Clone)] pub struct Blacklisted<T> {t: T, pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>> }"
//
template<class T>
struct Blacklisted {
	T t;
};

/// This would derive(Hash) if it didn't contain a blacklisted type,
/// causing us to conservatively avoid deriving hash for it.
struct WhitelistedOne {
	Blacklisted<int> a;
};

/// This can't derive(Hash) even if it didn't contain a blacklisted type.
struct WhitelistedTwo {
	Blacklisted<float> b;
};
