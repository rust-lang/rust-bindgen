// bindgen-flags: --no-layout-tests --default-visibility=private 

struct Point {
    int x;
    int y;
};

struct Color {
    char r :1;
    char g :1;
    char b :1;
};
