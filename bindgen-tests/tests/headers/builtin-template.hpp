namespace std {
template<class T, T... Ints>
class integer_sequence;
template<class T, T N>
using make_integer_sequence = __make_integer_seq<integer_sequence, T, N>;
}
