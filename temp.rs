
#[test]
fn volkertest() {
    dbg!("test");
    let bindings = bindgen::Builder::default()
        //.header("bintest/example.cpp")
        .header("bindgen-integration/cpp/Test.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bindings.rs")
        .expect("Couldn't write bindings!");
}






class OverloadedOperator {
    public:
        int val;
        int other;
        OverloadedOperator operator+(const OverloadedOperator& rhs) const;
        OverloadedOperator operator-(const OverloadedOperator& rhs) const;
        OverloadedOperator& operator+=(const OverloadedOperator& rhs);
        bool operator==(const OverloadedOperator& rhs) const ;
        int operator[](int index) const;
        OverloadedOperator operator-() const;
    };
    
    
    class MyClass {
    public:
        MyClass *ptr = 0;
        MyClass() {
            ptr = this;
        };
        ~MyClass() {
            //printf("destructing %x %x\n", this, ptr);
        };
    
        MyClass ( const MyClass & ) = delete;
        MyClass member(MyClass arg1, const MyClass arg2, MyClass *arg3, const MyClass *arg4/*, MyClass &arg5, const MyClass &arg6*/);
    };
        
        
        
    // class Wrap_MyClass{
    // public:
    //     MyClass *ptr = 0;
    //     Wrap_MyClass(MyClass *const ptrarg);
    //     Wrap_MyClass member(MyClass arg1, const MyClass arg2, MyClass * arg3, const MyClass * arg4);
    // };
    // Wrap_MyClass::Wrap_MyClass(MyClass *const ptrarg) {
    //     ptr = ptrarg;
    // }
    // Wrap_MyClass Wrap_MyClass::member(MyClass arg1, const MyClass arg2, MyClass * arg3, const MyClass * arg4) {
    //     auto val = new MyClass(this->ptr->member(arg1, arg2, arg3, arg4));
    //     return Wrap_MyClass(val);
    // }