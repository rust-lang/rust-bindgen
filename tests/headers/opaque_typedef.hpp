template<typename T>
class RandomTemplate;

/** <div rustbindgen opaque></div> */
typedef RandomTemplate<int> ShouldBeOpaque;

typedef RandomTemplate<int> ShouldNotBeOpaque;
