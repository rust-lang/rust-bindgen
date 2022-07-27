typedef unsigned char uint8_t;
typedef unsigned short uint16_t;

struct B {
  uint8_t a;
  uint16_t b;
  uint8_t c;
} __attribute__((aligned(2), packed));
