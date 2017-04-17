struct HasBigBitfield {
  __int128 x : 128;
};


struct HasTwoBigBitfields {
  __int128 x : 80;
  __int128 y : 48;
};
