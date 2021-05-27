/******************************************************************************

                              Online C++ Compiler.
               Code, Compile, Run and Debug C++ program online.
Write your code in this editor and press "Run" button to compile and execute it.

*******************************************************************************/

#include <iostream>
#include <cstdlib>
#include <cstddef>
#include <cstring>


        // priv(const priv&) {
        //    std::cout << "Copy construcing priv\n";
        // }


        // public:
        // priv(){
        //     std::cout << "Constructing priv\n";
        // }
        // priv& operator=(const priv& other) {
        //     std::cout << "Copy assigning priv\n";
        //     return *this;
        // }
        // ~priv(){
        //     std::cout << "Destructing priv\n";
        // }
    


class c {
    class priv{};
    public:
    typedef priv mytypedef; // Provides a way to construct priv
    static priv myfactory() {priv ret; return ret;} // Provides another way to construct priv
};

template <typename T>
auto destruct_or_throw(T* t) -> decltype(t->~T()) {
    t->~T();
}
auto destruct_or_throw(void*) -> void {
    std::abort();
}
void *c_mytypedef_wrapper() {
    auto ptr = (c::mytypedef *)malloc(123); // In my usecase, I know the size of the priv class
    c::mytypedef val;
    *ptr = val;
    return ptr;
}
void *c_myfactory_wrapper() {
    auto ptr = (decltype(c::myfactory()) *) malloc(123); // In my usecase, I know the size of the priv class
    auto val = c::myfactory();
    *ptr = val;
    return ptr;
}
void c_callable_destruct(void* t) {
    // destruct_or_throw((c::priv *) t); // compile error, because priv is private
    destruct_or_throw((decltype(c::myfactory()) *) t); // would work, but relies on us having the string "myfactory" when codegenerating this
    destruct_or_throw((c::mytypedef *) t); // would work, but relies on us having the string "mytypedef" when codegenerating this
    free(t);
}

int main() { // example use
    auto val1 = c_mytypedef_wrapper();
    c_callable_destruct(val1);

    auto val2 = c_myfactory_wrapper();
    c_callable_destruct(val2);
}

//extern template void priv_arg<c::priv>(c::priv* ptr);