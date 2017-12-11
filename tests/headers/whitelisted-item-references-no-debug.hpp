// bindgen-flags: --whitelist-type "WhitelistMe" --no-debug "NoDebug"

struct NoDebug {};

class WhitelistMe {
  NoDebug a;
};
