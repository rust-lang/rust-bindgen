struct Date {
    unsigned char day: 5;
    unsigned char month: 4;
    signed short year: 15;
} __attribute__((packed));
