// bindgen-flags: --blocklist-type RefPtr --raw-line '#[derive(Clone, Copy, Debug)] pub struct RefPtr<T>(T);' -- --std=c++14

// This is pretty much the same as the other issue 662 test case, but this time
// we blocklist RefPtr to exercise the instantiation-of-a-blocklisted-template
// path in the template analysis.

template <class> class RefPtr {};
template <class T> class nsMainThreadPtrHolder { T a; };
template <class U> class nsMainThreadPtrHandle {
    RefPtr<nsMainThreadPtrHolder<U>> mPtr;
};
