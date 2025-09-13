// bindgen-flags: --nonnull-references

struct Container {
  int *normalPointer;
  const int *constPointer;
  int &normalRef;
  const int &constRef;
  int *&pointerRef;
  const int *&constPointerRef;
};

int &refReturningFunction();

void functionConsumingRef(int &someRef, float normalArgument,
                          const int &constRef);

void functionConsumingPointerRef(int* &pointerRef);
