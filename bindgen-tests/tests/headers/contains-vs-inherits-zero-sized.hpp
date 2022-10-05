/**
 * This should get an `_address` byte.
 */
struct Empty {};

/**
 * This should not get an `_address` byte, so `sizeof(Inherits)` should be
 * `1`.
 */
struct Inherits : public Empty {
    bool b;
};

/**
 * This should not get an `_address` byte, but contains `Empty` which *does* get
 * one, so `sizeof(Contains)` should be `1 + 1`.
 */
struct Contains {
    Empty empty;
    bool b;
};
