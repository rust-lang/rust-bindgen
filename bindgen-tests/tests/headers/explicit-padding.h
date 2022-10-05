// bindgen-flags: --explicit-padding

typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;

struct pad_me {
        uint8_t first;
        uint32_t second;
        uint16_t third;
};

union dont_pad_me {
        uint8_t first;
        uint32_t second;
        uint16_t third;
};
