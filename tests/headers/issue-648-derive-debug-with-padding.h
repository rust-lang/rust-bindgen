/**
 * We emit a `[u8; 63usize]` padding field for this struct, which cannot derive
 * Debug because 63 is over the hard coded limit. (Yes, this struct doesn't end
 * up with the reight alignment, we're waiting on `#[repr(align="N")]` to land
 * in rustc).
 */
struct NoDebug {
    char c;
    // padding of 63 bytes
} __attribute__((__aligned__(64)));

/**
 * This should derive Debug because the padding size is less than the max derive
 * Debug impl for arrays. However, we conservatively don't derive Debug because
 * we determine Debug derive-ability before we compute padding, which happens at
 * codegen. (Again, we expect to get the alignment wrong for similar reasons.)
 */
struct ShouldDeriveDebugButDoesNot {
    char c[32];
    char d;
    // padding of 31 bytes
} __attribute__((__aligned__(64)));
