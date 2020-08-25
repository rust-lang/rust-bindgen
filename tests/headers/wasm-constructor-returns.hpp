// bindgen-flags: --generate constructors,types -- -fvisibility=default --target=wasm32-unknown-emscripten

class Foo {
public:
    Foo(int var);
};

