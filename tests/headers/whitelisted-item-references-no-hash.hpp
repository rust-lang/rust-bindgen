// bindgen-flags: --with-derive-hash --whitelist-type "WhitelistMe" --no-hash "NoHash"

struct NoHash {};

class WhitelistMe {
  NoHash a;
};
