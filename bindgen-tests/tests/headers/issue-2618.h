// bindgen-flags: --allowlist-var "val[0-9]+"

typedef __UINT32_TYPE__ uint32_t;
typedef __UINT64_TYPE__ uint64_t;

static const uint32_t val1 = 0x7fffffff;
static const uint32_t val2 = 0x80000000;
static const uint32_t val3 = 0xffffffff;
static const uint64_t val4 = 0x7fffffffffffffff;
static const uint64_t val5 = 0x8000000000000000;
static const uint64_t val6 = 0xffffffffffffffff;

static const uint32_t val7 = (0x7fffffff);
static const uint32_t val8 = (0x80000000);
static const uint32_t val9 = (0xffffffff);
static const uint64_t val10 = (0x7fffffffffffffff);
static const uint64_t val11 = (0x8000000000000000);
static const uint64_t val12 = (0xffffffffffffffff);
