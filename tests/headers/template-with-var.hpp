template<typename T>
class TemplateWithVar {
    // We shouldn't generate bindings for this because there are potentially
    // many instantiations of this variable, but we can't know which ones exist
    // or don't.
    static T var = 0;
};
