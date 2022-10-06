// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq --with-derive-partialord --with-derive-ord
// 
struct test {
    int a;
    char zero_length_array[0];
};

struct test2 {
    int a;
    char incomplete_array[];
};

struct test3 {
    int a;
    char zero_length_array[0];
    char incomplete_array[];
};
