// bindgen-flags: --default-enum-style newtype

enum pipe_resource_usage {
  PIPE_USAGE_DEFAULT,
  PIPE_USAGE_IMMUTABLE,
  PIPE_USAGE_DYNAMIC,
  PIPE_USAGE_STREAM,
  PIPE_USAGE_STAGING
};

struct pipe_resource {
  unsigned compression_rate : 4;
  enum pipe_resource_usage usage : 4;
};
