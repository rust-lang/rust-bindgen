// bindgen-flags: --use-specific-virtual-function-receiver --generate-inline-functions -- -x c++ -std=c++14

class Fish {
public:
    virtual void swim() {
    }
};
