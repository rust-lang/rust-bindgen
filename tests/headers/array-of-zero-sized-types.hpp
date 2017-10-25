/**
 * This should get an `_address` byte.
 */
struct Empty {};

/**
 * This should not get an `_address` byte, since each `Empty` gets one, meaning
 * that this object is addressable.
 */
struct HasArrayOfEmpty {
    Empty empties[10];
};
