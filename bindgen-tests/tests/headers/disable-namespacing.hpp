// bindgen-flags: --disable-name-namespacing

namespace foo {
namespace bar {

typedef int Baz;

// anonymous structs should still be "namespaced"
struct Foo {
	struct {
		int a;
	} anon;
};

}
}
