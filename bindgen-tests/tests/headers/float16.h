// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq

static __fp16 global;

struct Test__Float16
{
    __fp16 f;
};

struct Test__Float16Ref
{
    __fp16 *f;
};

/*
// This options are currently supported only on specific targets (eg. x86 with sse2)
_Float16 returns_f16();

void gets_f16(_Float16 arg);

struct Test__Float16_Complex
{
    _Float16 _Complex mMember;
};

struct Test__Float16_ComplexPtr
{
    _Float16 _Complex *mMember;
};

_Float16 _Complex globalValueHalf;

_Float16 _Complex returns_f16_complex();

void gets_f16_complex(_Float16 _Complex arg);
*/