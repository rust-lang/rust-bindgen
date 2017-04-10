// bindgen-flags: --blacklist-type RefPtr --whitelist-function 'Servo_.*' --raw-line 'pub type RefPtr<T> = T;' -- -std=c++14
template <class> class RefPtr;
class b;
class A {
  typedef b a;
};
template <class c> class e { RefPtr<c> d; };
template <class> class f {};
class g {
  f<e<int>> h;
};
class b : g {};
A Servo_Element_GetSnapshot();
