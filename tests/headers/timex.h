struct timex {
	int tai;

	int  :32; int  :32; int  :32; int  :32;
	int  :32; int  :32; int  :32; int  :32;
	int  :32; int  :32; int  :32;
};

struct timex_named {
  int tai;

  int a:32; int b:32; int c:32; int d:32;
  int e:32; int f:32; int g:32; int h:32;
  int i:32; int j:32; int k:32;
};
