// bindgen-flags: --with-derive-partialeq --whitelist-type "WhitelistMe" --no-partialeq "NoPartialEq"

struct NoPartialEq {};

class WhitelistMe {
  NoPartialEq a;
};
