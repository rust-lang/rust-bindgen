class VirtualMethods {
  virtual void foo();
};

template<typename K>
class Set {
  int bar;
};

class ServoElementSnapshotTable
  : public Set<VirtualMethods>
{};
