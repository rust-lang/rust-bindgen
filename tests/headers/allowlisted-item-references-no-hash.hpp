// bindgen-flags: --with-derive-hash --allowlist-type "AllowlistMe" --no-hash "NoHash"

struct NoHash {};

class AllowlistMe {
  NoHash a;
};
