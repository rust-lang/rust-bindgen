// bindgen-flags: --enable-cxx-namespaces

/**
 * This is a multi-line doc comment.
 *
 * This class is really really interesting, look!
 */
class Foo {
  /**
   * This nested class is also a multi-line doc comment.
   *
   * This class is not so interesting, but worth a bit of docs too!
   */
  class Bar { };
};

namespace test {
  /**
   * I'm in a namespace, and thus I may be on a rust module, most of the time.
   * My documentation is pretty extensive, I guess.
   */
  class Baz {
    /**
     * This member is plain awesome, just amazing.
     *
     * It also has super-extensive docs, with even a nice ascii-art diagram.
     *
     * +------+          +-------+
     * | foo  |   ---->  | bar   |
     * +------+          +-------+
     */
    int member;
  };

  inline namespace foobiedoobie {
    /**
     * I'm in an inline namespace, and as such I shouldn't get generated inside
     * a rust module, except when the relevant option is specified. Also, this
     * comment shouldn't be misaligned.
     */
    class InInlineNS {
    };
  }

  /**/
  class Bazz {};
}
