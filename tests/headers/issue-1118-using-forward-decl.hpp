template <class> class nsTArray;
template <class b> using c = nsTArray<b>;
class nsTArray_base {
  int *d;
};
template <class> class nsTArray : nsTArray_base {};
class nsIContent {
  nsTArray<nsIContent *> foo;
};
nsTArray<nsIContent*> *Gecko_GetAnonymousContentForElement();
