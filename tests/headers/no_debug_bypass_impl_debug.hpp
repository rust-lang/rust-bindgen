// bindgen-flags: --no-debug "NoDebug" --impl-debug

template<typename T>
class Generic {
  T t[40];
};

template<typename T>
class NoDebug {
  T t[40];
};
