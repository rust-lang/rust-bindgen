// bindgen-flags: --default-visibility private --no-doc-comments

struct my_struct1 {
    int a: 1;
};

/** <div rustbindgen private="false"></div> */
struct my_struct2 {
    int a: 1;
};
