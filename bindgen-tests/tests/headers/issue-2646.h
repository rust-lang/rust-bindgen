// bindgen-flags: --rustified-enum 'Plain.*' --rustified-enum 'TryFromRaw.*=try_from_raw' --rustified-enum='FromRawUnchecked.*=from_raw_unchecked' --rustified-enum='Both.*=try_from_raw,from_raw_unchecked' --rustified-enum 'NonExhaustive.*=non_exhaustive'

enum Plain {
    Plain1,
    Plain2,
    Plain3
};

enum TryFromRaw {
    TFR1 = -1,
    TFR2 = 5,
    TFR3
};

enum FromRawUnchecked {
    FRU1 = 6,
    FRU2 = 10,
    FRU3 = 11,
};

enum Both {
    Both1,
    Both2 = -1,
    Both3,
};

enum NonExhaustive {
    Ex1,
    Ex2,
};
