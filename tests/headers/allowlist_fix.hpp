// bindgen-flags: --allowlist-function 'Servo_.*' --blocklist-type Test --raw-line "pub enum Test {}"

struct Test {};
extern "C" void Servo_Test(Test* a);
