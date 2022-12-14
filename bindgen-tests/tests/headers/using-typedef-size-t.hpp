// #define WORKS

namespace std {
typedef unsigned long size_t;
}
using std::size_t;

namespace mozilla {
class MarkerSchema;
}

extern "C" {
void gecko_profiler_marker_schema_set_chart_label(
    mozilla::MarkerSchema* schema, const char* label, size_t len);
}
