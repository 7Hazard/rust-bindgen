enum Foo: unsigned char {
    Bar = 0,
    Qux
};

enum Neg: char {
    MinusOne = -1,
    One = 1,
};

enum Bigger: unsigned short {
    Much = 255,
    Larger
};

enum MuchLong: long {
    MuchLow = -4294967296,
};

enum MuchLongLong: unsigned long long {
    MuchHigh = 4294967296,
};

enum MostLongLong: long long {
    MostLow = -9223372036854775808,
};
