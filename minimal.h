#include <limits.h>
#include <stdint.h>

#define MY_USHORT (unsigned short) ULONG_MAX

typedef unsigned long TickType_t;
#define portMAX_DELAY (TickType_t) ULONG_MAX
const TickType_t __portMAX_DELAY = portMAX_DELAY;
