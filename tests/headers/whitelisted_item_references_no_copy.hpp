// bindgen-flags: --whitelist-type "WhitelistMe" --no-copy "NoCopy"

struct NoCopy {};

class WhitelistMe {
  NoCopy a;
};
