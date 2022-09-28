typedef struct foo {
  int type;
  long type_;
  long long type__;
};

enum Type {
    let,
    match,
    match_,
};

int type(enum Type type_, enum Type type) { return 1; }
