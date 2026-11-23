// bindgen-flags: --enable-cxx-namespaces

namespace ns {
extern int array[];
extern int array[3];
}

extern "C" {
extern int c_array[];
extern int c_array[4];
}

extern int defined[];
int defined[5] = {0};
