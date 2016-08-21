template<typename...> using __void_t = void;

template<typename _Iterator, typename = __void_t<>>
  struct __iterator_traits { };
