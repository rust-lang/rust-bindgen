/// We don't get a layout for this bitfield, since we don't know what `T` will
/// be, so we cannot allocate bitfield units. The best thing we can do is make
/// the struct opaque.
template <class T>
class TemplatizedBitfield {
    T t : 6;
};
