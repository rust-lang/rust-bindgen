template<class K, class V>
class Entry : public K
{
  V mData;
};

template<typename K, typename V>
class nsBaseHashtable {
  typedef Entry<K, V> EntryType;

  struct EntryPtr {
  private:
    EntryType& mEntry;
    bool mExistingEntry;
  };
};
