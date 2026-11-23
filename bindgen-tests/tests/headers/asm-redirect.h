
void func_a();
void func_a() __asm("func_a_redir");

void func_b();
extern __typeof(func_b) func_b __asm("func_b_redir");

void func_c();
void func_c();
void func_c() __asm("func_c_redir");

void func_d() __asm("func_d_redir");
