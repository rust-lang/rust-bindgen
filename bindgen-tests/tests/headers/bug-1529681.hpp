// bindgen-flags: -- -std=c++14
//
// https://bugzilla.mozilla.org/show_bug.cgi?id=1529681
// https://bugs.llvm.org/show_bug.cgi?id=40813

class BrowsingContext {
  auto Tie(void* aUnused) const;
};
