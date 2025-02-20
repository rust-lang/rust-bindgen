// bindgen-flags: --generate-deleted-functions --generate-private-functions --generate-pure-virtual-functions --generate-inline-functions -- -x c++ -std=c++14

class Test {
public:
    virtual void a() = 0;
    void b() = delete;
private:
    void c() {}
};
