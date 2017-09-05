// bindgen-flags: --with-derive-hash --blacklist-type BlacklistMe --raw-line 'pub struct BlacklistMe(u8);'

struct BlacklistMe {};

/**
 * Because this type contains a blacklisted type, it should not derive Hash.
 */
struct ShouldNotDeriveHash {
    BlacklistMe a;
};
