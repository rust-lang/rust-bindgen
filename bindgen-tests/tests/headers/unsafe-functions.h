// bindgen-flags: --safe-functions is_zero --safe-functions '^is_number_\d+$'

int is_zero(int n);
int is_zero_ptr(int *n);
int is_number_5(int n);
int is_number_42(int n);
int is_number_other_than_8(int n);
