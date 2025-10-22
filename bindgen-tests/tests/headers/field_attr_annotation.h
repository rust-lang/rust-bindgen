/// <div rustbindgen deriveDebug></div>
struct Point {
    /// <div rustbindgen attribute="cfg(test)"></div>
    int x;
    /// <div rustbindgen attribute="allow(dead_code)"></div>
    int y;
};

/// <div rustbindgen deriveDebug></div>
union Data {
    /// <div rustbindgen attribute="allow(dead_code)"></div>
    int i;
    float f;
};

/// <div rustbindgen attribute="cfg(test)"></div>
typedef int Handle;
