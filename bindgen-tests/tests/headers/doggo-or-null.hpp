// bindgen-flags: --opaque-type DoggoOrNull --with-derive-partialeq --with-derive-hash -- -std=c++14

class Doggo {
    int x;
};

class Null {};

/**
 * This type is an opaque union. Unions can't derive anything interesting like
 * Debug or Default, even if their layout can, because it would require knowing
 * which variant is in use. Opaque unions still end up as a `union` in the Rust
 * bindings, but they just have one variant. Even so, can't derive. We should
 * probably emit an opaque struct for opaque unions... but until then, we have
 * this test to make sure that opaque unions don't derive and still compile.
 */
union DoggoOrNull {
    Doggo doggo;
    Null none;
};
