// bindgen-flags: --represent-cxx-operators --generate-inline-functions -- -x c++
// bindgen-parse-callbacks: operator-rename

class SomeClass {
public:
    bool operator=(const SomeClass& another) {
        return false;
    }
};
