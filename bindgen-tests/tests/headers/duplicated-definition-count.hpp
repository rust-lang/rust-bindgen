class BitStream {
 public:
  void Write(const char *inputByteArray, unsigned int numberOfBytes);
  void Write(BitStream *bitStream, unsigned numberOfBits);
  void Write1();
};
