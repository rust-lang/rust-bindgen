// bindgen-flags: -- --std=c++14

template <class T> class RefPtr { T a; };
template <class T> class nsMainThreadPtrHolder { T a; };
template <class T> class nsMainThreadPtrHandle {
    RefPtr<nsMainThreadPtrHolder<T>> mPtr;
};
