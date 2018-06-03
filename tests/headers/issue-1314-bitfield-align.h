typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned uint32_t;

typedef struct {
  uint8_t a;
  uint32_t b : 15;
} S;

typedef struct {
  uint8_t a;
  uint16_t b : 15;
} S2;
