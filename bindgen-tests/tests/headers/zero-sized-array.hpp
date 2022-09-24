// These classes are technically zero-sized, but despite that they still don't
// get an `_address` field inserted.

/**
 * Bizarrely enough, this should *not* get an `_address` field.
 */
class ZeroSizedArray {
    char arr[0];
};

/**
 * And nor should this get an `_address` field.
 */
class ContainsZeroSizedArray {
    ZeroSizedArray zsa;
};

/**
 * Inheriting from ZeroSizedArray shouldn't cause an `_address` to be inserted
 * either.
 */
class InheritsZeroSizedArray : ZeroSizedArray {};

// These are dynamically sized, which means that `sizeof` yields `0` but it
// isn't really true. We shouldn't add an `_address` field to them.

/**
 * And this should not get an `_address` field either.
 */
class DynamicallySizedArray {
    char arr[];
};

/**
 * No `_address` field here either.
 */
class ContainsDynamicallySizedArray {
    DynamicallySizedArray dsa;
};

// Note: this is disallowed:
//
//     error: base class 'DynamicallySizedArray' has a flexible array member
//
// class InheritsDynamicallySizedArray : DynamicallySizedArray {};
