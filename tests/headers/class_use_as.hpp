// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq

/**
 * <div rustbindgen="true" replaces="whatever"></div>
 */
struct whatever_replacement {
    int replacement;
};

struct whatever {
    int b;
};

struct container {
    whatever c;
};
