// bindgen-flags: -- -std=c++14

using uint32_t = unsigned long;
using size_t = unsigned long long;

template<class KeyClass, class DataType>
class nsBaseHashtableET {
};

template<class Entry>
class nsTHashtable {
};

template<class KeyClass, class DataType, class UserDataType>
class nsBaseHashtable
  : protected nsTHashtable<nsBaseHashtableET<KeyClass, DataType>>
{

public:
  typedef typename KeyClass::KeyType KeyType;
  typedef nsBaseHashtableET<KeyClass, DataType> EntryType;

  using nsTHashtable<EntryType>::Contains;
  using nsTHashtable<EntryType>::GetGeneration;

  nsBaseHashtable() {}
  explicit nsBaseHashtable(uint32_t);

  struct LookupResult {
  private:
    EntryType* mEntry;
    nsBaseHashtable& mTable;

  public:
    LookupResult(EntryType*, nsBaseHashtable&);
  };

  struct EntryPtr {
  private:
    EntryType& mEntry;
    bool mExistingEntry;

  public:
    EntryPtr(nsBaseHashtable&, EntryType*, bool);
    ~EntryPtr();
  };

};
