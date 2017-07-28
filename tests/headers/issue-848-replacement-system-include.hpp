// bindgen-flags: -- -Itests/headers/issue-848

#include "an-include.h"

extern "C" {
  nsTArray<int>* func();
}
