// bindgen-flags: --with-derive-partialeq --allowlist-type "AllowlistMe" --no-partialeq "NoPartialEq"

struct NoPartialEq {};

class AllowlistMe {
  NoPartialEq a;
};
