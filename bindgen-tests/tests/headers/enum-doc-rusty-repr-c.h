// bindgen-flags: --rustified-repr-c-enum B|BigEnum

#include "enum-doc.h"

/** An enum with a value larger than the platform's `c_int` */
enum BigEnum {
    /** A value that is too large to fit in a 32-bit integer */
    BIG_ENUM_BIG = 4294967296,
};
