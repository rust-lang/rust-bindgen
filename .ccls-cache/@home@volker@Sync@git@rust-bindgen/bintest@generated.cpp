#include "example.hpp"
void bindgen_destruct_Member(Member *ptr){
                ptr->~Member();
            }
void bindgen_destruct_Base(Base *ptr){
                ptr->~Base();
            }
