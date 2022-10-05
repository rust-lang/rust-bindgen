// bindgen-flags: --with-derive-partialeq --blocklist-type BlocklistMe --raw-line 'pub struct BlocklistMe(u8);'

struct BlocklistMe {};

/**
 * Because this type contains a blocklisted type, it should not derive
 * PartialEq.
 */
struct ShouldNotDerivePartialEq {
    BlocklistMe a;
};
