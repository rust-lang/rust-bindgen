// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq --impl-partialeq --rust-target 1.40
/**
 * We emit a `[u8; 63usize]` padding field for this struct, which cannot derive
 * Debug/Hash because 63 is over the hard coded limit.
 */
struct NoDebug {
    char c;
    // padding of 63 bytes
} __attribute__((__aligned__(64)));

/**
 * This should derive Debug/Hash/PartialEq/Eq because the padding size is less than the max derive
 * Debug/Hash/PartialEq/Eq impl for arrays. However, we conservatively don't derive Debug/Hash because
 * we determine Debug derive-ability before we compute padding, which happens at
 * codegen.
 */
struct ShouldDeriveDebugButDoesNot {
    char c[32];
    char d;
    // padding of 31 bytes
} __attribute__((__aligned__(64)));
