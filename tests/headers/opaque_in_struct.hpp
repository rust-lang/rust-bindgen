// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq


/** <div rustbindgen opaque> */
typedef struct opaque {
    int waht;
} opaque;

struct container {
    opaque contained;
};
