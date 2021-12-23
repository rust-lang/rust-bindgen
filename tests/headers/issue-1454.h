// bindgen-flags: --no-recursive-allowlist --allowlist-type "local_type" --with-derive-hash --no-derive-copy --no-derive-default --raw-line "#[repr(C)] #[derive(Debug)] pub struct extern_type;"
// bindgen-parse-callbacks: blocklisted-type-implements-trait

struct extern_type {};

typedef struct
{
    struct extern_type inner;
}
local_type;
