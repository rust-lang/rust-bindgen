// bindgen-flags: --new-type-alias (Foo|Bar|Baz|Bang)

// Fake stdint.h and stdbool.h
typedef __UINT64_TYPE__ uint64_t;
#define bool _Bool
#define true 1

typedef uint64_t Foo;
static const Foo Foo_A = 1;

typedef char Bar;
static const Bar Bar_A = 'a';

typedef float Baz;
static const Baz Baz_A = 3.25;

typedef bool Bang;
static const Bang Bang_A = true;

// Not wrapped
typedef uint64_t Boom;
static const Boom Boom_A = 2;
