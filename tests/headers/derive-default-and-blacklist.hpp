// bindgen-flags: --blacklist-type BlacklistMe --raw-line 'pub struct BlacklistMe(u8);'

// Note that we do not explicitly provide the `--with-derive-default` flag
// above, since it is added by the test runner implicitly.

struct BlacklistMe {};

/**
 * Because this type contains a blacklisted type, it should not derive
 * Default. Instead, we should emit a `mem::zeroed` implementation.
 */
struct ShouldNotDeriveDefault {
    BlacklistMe a;
};
