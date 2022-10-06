// bindgen-flags: --allowlist-file ".*/allowlisted/file.*" --allowlist-type AllowlistMe -- -Itests/headers


// Forward declaration of struct that's defined in an allowlisted file.
struct StructWithAllowlistedDefinition;

#include "allowlisted/file.hpp"

// Actual definition of struct that has a forward declaration in an allowlisted file.
struct StructWithAllowlistedFwdDecl {
    int b;
};

class Ignored {
    char c;
};

// Also have an explicitly allowlisted type
class AllowlistMe {
    int foo;
};
