// bindgen-flags: --rust-target 1.40

struct nsFoo {
  float details[400];
};

extern const struct nsFoo gDetails;
