template <class a> class b {
    typedef a td;
    using ta = a;
    struct foo {
        a foo : sizeof(a);
        a : sizeof(a);
        td : sizeof(td);
        ta : sizeof(ta);
    };
};
