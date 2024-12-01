// bindgen-flags: --merge-extern-blocks --enable-cxx-namespaces --rust-target=1.81 -- --target=x86_64-unknown-linux
int foo();
typedef struct Point {
    int x;
} Point;
int bar();

namespace ns {
    int foo();
    typedef struct Point {
        int x;
    } Point;
    int bar();
}
