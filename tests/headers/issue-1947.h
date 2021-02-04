typedef unsigned char U8;
typedef unsigned short U16;

typedef struct {
    U16 MADZ : 10, MAI0 : 2, MAI1 : 2, MAI2 : 2;
    U8 MADK, MABR;
    U16 MATH : 10, MATE : 4, MATW : 2;
    U8 MASW : 4, MABW : 3, MAXN : 1, _rB_;
} V56AMDY;
