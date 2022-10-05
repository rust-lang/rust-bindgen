// bindgen-flags: --allowlist-type "AllowlistMe" --no-copy "NoCopy"

struct NoCopy {};

class AllowlistMe {
  NoCopy a;
};
