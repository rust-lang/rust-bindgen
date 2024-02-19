typedef struct {
    int field;
} strct;

typedef strct typ[1];

extern typ w;
extern strct *x;

extern const typ y;
extern const strct *z;

void function(const typ a, const strct *b);
