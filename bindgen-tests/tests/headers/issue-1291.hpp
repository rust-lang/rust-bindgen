// bindgen-flags: --rust-target 1.25
// bindgen-unstable

struct __attribute__((aligned(16))) RTCRay
    {
  float org[3];
  float align0;
  float dir[3];
  float align1;
  float tnear;
  float tfar;
  float time;
  unsigned mask;
  float Ng[3];
  float align2;
  float u;
  float v;
  unsigned geomID;
  unsigned primID;
  unsigned instID;
};
