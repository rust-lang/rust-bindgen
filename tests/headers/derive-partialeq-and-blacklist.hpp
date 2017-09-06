// bindgen-flags: --with-derive-partialeq --blacklist-type BlacklistMe --raw-line 'pub struct BlacklistMe(u8);'

struct BlacklistMe {};

/**
 * Because this type contains a blacklisted type, it should not derive
 * PartialEq.
 */
struct ShouldNotDerivePartialEq {
    BlacklistMe a;
};
