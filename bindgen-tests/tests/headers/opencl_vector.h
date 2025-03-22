typedef float float4 __attribute__((ext_vector_type(4)));
typedef float float2 __attribute__((ext_vector_type(2)));

float4 foo(float2 a, float2 b) {
  float4 c;
  c.xz = a;
  c.yw = b;
  return c;
}