
class NoTemplateParams {
public:
    int a;
};

template<class T>
class UsesTemplateParam {
public:
    UsesTemplateParam(T);
    T t;
};

template<class A>
class IgnoresTemplateParam {
public:
    IgnoresTemplateParam();
    int a;
};

typedef NoTemplateParams TypedefNoTemplateParam;
using TypedefNoTemplateParam2 = NoTemplateParams;
typedef UsesTemplateParam<int> TypedefUsesTemplateParam;
using TypedefUsesTemplateParam2 = UsesTemplateParam<int>;
template <class T>
using TypedefUsesTemplateParam3 = UsesTemplateParam<T>;
typedef IgnoresTemplateParam<int> TypedefIgnoresTemplateParam;
template <class T>
using TypedefIgnoresTemplateParam2 = IgnoresTemplateParam<T>;