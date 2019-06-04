extern bool Evaluate(char r);
extern bool Evaluate(int x, int y);

extern void CanonicalLast(char r);
extern void CanonicalLast(void);

extern float Common(int*, int y);
extern float Common(int*, bool *y);
extern float Common(int*, const char *y);

extern void CanonicalLastWithCommon(int*, int y);
extern void CanonicalLastWithCommon(int*);

namespace foo {
    extern void MyFunction();
}
namespace bar {
    extern void MyFunction();
}
