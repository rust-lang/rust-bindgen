// bindgen-flags: -- -std=c++20

extern char8_t global_array[4];
extern char8_t incomplete_array[];
extern char8_t nested_array[2][2];

struct WithChar8Array {
  char8_t bytes[8];
  char8_t single;
};

void take_char8_array(char8_t input[4]);
char8_t return_char8(char8_t input);

enum class Char8Enum : char8_t {
  High = 0xFF,
  Low = 0x41,
};
