// bindgen-flags: --no-recursive-allowlist --allowlist-type "my_rlimit_conf" --derive-ignore-blocklist --raw-line "#[repr(C)] #[derive(Debug, Default, Copy, Clone)] pub struct rlimit;"

struct rlimit {};

typedef struct
{
    struct rlimit core;
    struct rlimit cpu;
    struct rlimit data;
    struct rlimit fsize;
}
my_rlimit_conf;
