typedef
    void __stdcall
    EVT_VIGEM_X360_NOTIFICATION(
        void* Client,
        void* Target,
        unsigned char LargeMotor,
        unsigned char SmallMotor,
        unsigned char LedNumber,
        void* UserData
    );

typedef EVT_VIGEM_X360_NOTIFICATION *PFN_VIGEM_X360_NOTIFICATION;
