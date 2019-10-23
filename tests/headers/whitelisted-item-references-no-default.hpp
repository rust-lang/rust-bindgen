// bindgen-flags: --no-default "NoDefault" --whitelist-type "Whitelisted"

// `--with-derive-default` is added by the test runner implicitly.

struct NoDefault {};

class Whitelisted {
    NoDefault x;
};
