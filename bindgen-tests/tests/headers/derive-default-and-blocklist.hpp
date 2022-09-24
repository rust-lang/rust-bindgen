// bindgen-flags: --blocklist-type BlocklistMe --raw-line 'pub struct BlocklistMe(u8);'

// Note that we do not explicitly provide the `--with-derive-default` flag
// above, since it is added by the test runner implicitly.

struct BlocklistMe {};

/**
 * Because this type contains a blocklisted type, it should not derive
 * Default. Instead, we should emit a `mem::zeroed` implementation.
 */
struct ShouldNotDeriveDefault {
    BlocklistMe a;
};
