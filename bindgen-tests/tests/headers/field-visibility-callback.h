// bindgen-flags: --default-visibility private
// bindgen-parse-callbacks: field-visibility-default-private

struct my_struct {
    int a;
    int private_b;
    int c: 1;
    int private_d: 1;
};
