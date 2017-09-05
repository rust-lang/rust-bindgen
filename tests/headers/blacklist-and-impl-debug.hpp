// bindgen-flags: --impl-debug --blacklist-type BlacklistMe --raw-line 'pub struct BlacklistMe(u8);'

struct BlacklistMe {};

/**
 * Because this type contains a blacklisted type, it should not derive Debug.
 */
struct ShouldManuallyImplDebug {
    BlacklistMe a;
};
