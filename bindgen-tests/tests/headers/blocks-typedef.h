// bindgen-flags: --generate-block --block-extern-crate -- -fblocks
// bindgen-osx-only

typedef void F(int);
typedef F (^Blk);
int take(Blk b, int t);

typedef void (^DirectBlk)(int);
int direct(DirectBlk b, int t);
