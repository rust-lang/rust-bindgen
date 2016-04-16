class C {
    int a;
    // More than rust limits (32)
    char big_array[33];
};

class WithDtor {
    int b;

    ~WithDtor() {}
};

union Union {
    float d;
    int i;
};

class WithUnion {
    Union data;
};
