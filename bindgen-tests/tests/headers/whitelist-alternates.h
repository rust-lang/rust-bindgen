// bindgen-flags: --whitelist-type 'Whitelisted.*|Allow.*'
// Test for changes introduced in #1756

struct WhitelistedType {};
struct AllowType {};
// this would have been accepted because the start anchor
// wouldn't be applied to the second alternative:
struct NoAllowType {};
